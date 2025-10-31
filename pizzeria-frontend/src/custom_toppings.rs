use std::error::Error;
use std::io;
use std::io::{BufRead, BufReader, Read, Stdin, Stdout, Write};
use std::net::TcpStream;
use crate::clear_screen;
use crate::error::FrontendError;
use crate::table::{Align, Table, TableCell, TableRow};
use crate::table_menu::TableMenu;
use crate::http::{backend_socket_addr, read_toppings};
use crate::http::request::RequestBuilder;
use crate::ui::{wait_enter, prompt};

// Entfernen nach Nummer oder Name
pub fn remove_topping(stdout: &mut Stdout, stdin: &mut Stdin, _path: &str) -> io::Result<()> {

    let body = read_toppings()?;

    list_toppings_from_str(stdout, &body)?;

    let lines: Vec<String> = body
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();

    let choice = prompt(stdin, stdout, "\nEintrag löschen (Nummer oder Name, 'q' zum Abbrechen): ")?;
    let choice = choice.trim();
    if choice.eq_ignore_ascii_case("q") || choice.is_empty() {
        writeln!(stdout, "Abgebrochen.")?;
        return Ok(());
    }

    if lines.is_empty() {
        writeln!(stdout, "Keine Toppings vorhanden.")?;
        stdout.flush()?;
        wait_enter(stdout, stdin, "\n[Weiter mit Enter]")?;
        return Ok(());
    }

    // Auswahl interpretieren
    let name_to_delete: String = if let Ok(idx1) = choice.parse::<usize>() {
        // Nummernbasiert
        if !(1..=lines.len()).contains(&idx1) {
            writeln!(stdout, "Ungültige Nummer.")?;
            stdout.flush()?;
            wait_enter(stdout, stdin, "\n[Weiter mit Enter]")?;
            return Ok(());
        }
        let entry = &lines[idx1 - 1];
        entry.split('#').next().unwrap_or("").to_string()
    } else {
        // Namensbasiert: suche ersten Eintrag vor '#'
        if let Some(pos) = lines
            .iter()
            .position(|l| l.split('#').next().unwrap_or("")
            .eq_ignore_ascii_case(choice))
        {
            lines[pos].split('#').next().unwrap_or("").to_string()
        } else {
            writeln!(stdout, "Kein Eintrag mit diesem Namen gefunden.")?;
            stdout.flush()?;
            wait_enter(stdout, stdin, "\n[Weiter mit Enter]")?;
            return Ok(());
        }
    };

        send_delete_topping(&name_to_delete)?;
        clear_screen(stdout).expect("Should clear screen.");
    
        //neu laden und anzeigen
        let body_after = read_toppings()?;
        list_toppings_from_str(stdout, &body_after)?;

        writeln!(stdout, "\x1b[1;31mEntfernt:\x1b[0m \x1b[1m{name}\x1b[0m", name = name_to_delete)?;
        stdout.flush()?;

    Ok(())
}

pub fn add_toppings(stdout: &mut Stdout, stdin: &mut Stdin) -> Result<(), Box<dyn Error>> {

    loop {
        clear_screen(stdout)?;
        writeln!(stdout, "\x1b[1;31mTopping hinzufügen\x1b[0m (Name, dann Preis). 'q' zum Abbrechen.")?;
        stdout.flush()?;

        let topping_name = {
            let input = prompt(stdin, stdout, "\x1b[4;34mName\x1b[0m: ")?;
            let name = input.trim();
            if name.is_empty() || name.eq_ignore_ascii_case("q") {
                writeln!(stdout, "Abgebrochen.")?;
                return Ok(());
            }
            name.to_string()
        };

        let topping_price: u32 = loop {
            let input = prompt(stdin, stdout, "\x1b[4;34mPreis (Ganzzahl)\x1b[0m: ")?;
            let input = input.trim();

            if input.eq_ignore_ascii_case("q") || input.is_empty() {
                writeln!(stdout, "Abgebrochen.")?;
                return Ok(());
            }

            match input.parse::<u32>() {
                Ok(n) => break n,
                Err(_) => {
                    writeln!(stdout, "Ungültiger Preis. Bitte Ganzzahl angeben.")?;
                    continue;
                }
            }
        };

        let line = format!("{}#{}", topping_name, topping_price);
        // if !line.ends_with('\n') { line.push('\n'); }

        send_post("/toppings", &line)?;

        writeln!(stdout, "\nErfolgreich hinzugefügt: \x1b[1;32m{} {}\x1b[0m", topping_name, topping_price)?;
        stdout.flush()?;

        let again = prompt(stdin, stdout, "Weiteres Topping hinzufügen? (j/n): ")?;
        let again = again.trim().to_lowercase();
        if again != "j" && again != "ja" {
            break;
        }
    }
    Ok(())
}

fn send_post(path: &str, body: &str) -> io::Result<()> {
    let addr = backend_socket_addr()
        .map_err(FrontendError::into_io)?;
    let mut stream =  TcpStream::connect(addr)?;
    let body_length = body.as_bytes().len();

    let request = RequestBuilder::post()
        .path(String::from(path))
        .host(addr.to_string())
        .content_type(String::from("text/plain; charset=utf-8"))
        .content_length(body_length)
        .body(String::from(body))
        .build();

    stream.write_all(request.as_bytes())?;
    stream.flush()?;

    let mut resp = String::new();
    stream.read_to_string(&mut resp).ok();
    let code = resp.split_whitespace()
        .nth(1)
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(0);

    ensure_success_code(code)
}

fn send_delete_topping(name: &str) -> io::Result<()> {
    use std::io::Write;
    use std::net::TcpStream;

    let name_enc = urlencoding::encode(name);
    let addr = backend_socket_addr()
        .map_err(FrontendError::into_io)?;
    let mut stream =  TcpStream::connect(addr)?;

    let request = RequestBuilder::delete()
        .path(format!("/toppings?name={name_enc}"))
        .host(addr.to_string())
        .build();

    stream.write_all(request.as_bytes())?;
    stream.flush()?;

    let mut reader = BufReader::new(stream);
    let code = http_read_status(&mut reader)?;

    ensure_success_code(code)
}

pub fn send_clear_toppings(path: &str) -> io::Result<()> {
    let addr = backend_socket_addr()
        .map_err(FrontendError::into_io)?;
    let mut stream =  TcpStream::connect(addr)?;

    let request = RequestBuilder::delete()
        .path(String::from(path))
        .host(addr.to_string())
        .build();

    stream.write_all(request.as_bytes())?;
    stream.flush()?;

    let mut reader = BufReader::new(stream);
    let code = http_read_status(&mut reader)?;

    ensure_success_code(code)
}

fn list_toppings_from_str(stdout: &mut Stdout, content: &str) -> io::Result<()> {
    let title_text = String::from("Aktuelle Toppings");
    let mut table = Table::new(vec![]);

    for (index, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() { continue; }

        let mut parts = line.split('#');
        let name  = parts.next().unwrap_or("").trim();
        let price = parts.next().unwrap_or("").trim();

        table.push(TableRow::new(vec![
            TableCell::new(format!("{}.", index + 1)),
            TableCell::new(name.to_string()),
            TableCell::new_with_alignment(format!("{}.00$", price), Align::Right),
        ]));
    }

    if table.is_empty() {
        let table = Table::new(vec![
            TableRow::new(vec![TableCell::new(String::from("Noch keine Toppings vorhanden!"))]),
        ]);
        let table_menu = TableMenu::new(title_text, table);
        writeln!(stdout, "{table_menu}")?;
        stdout.flush()?;
        return Ok(());
    }

    let table_menu = TableMenu::new(title_text, table);
    writeln!(stdout, "{table_menu}")?;
    stdout.flush()?;
    Ok(())
}

pub fn list_toppings_from_backend(stdout: &mut Stdout) -> io::Result<()> {
    let body = read_toppings()?;
    list_toppings_from_str(stdout, &body)
}

fn http_read_status<R: BufRead>(reader: &mut R) -> io::Result<u16> {
    let mut status_line = String::new();
    reader.read_line(&mut status_line)?;

    let code = status_line
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(0);

    let mut line = String::new();
    loop {
        line.clear();
        let n = reader.read_line(&mut line)?;
        if n == 0 { break; }
        if line == "\r\n" || line.trim().is_empty() { break; }
    }

    Ok(code)
}

fn ensure_success_code(code:u16) -> io::Result<()> {
    if (200..300).contains(&code) {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, format!("HTTP {code}")))
    }
}
