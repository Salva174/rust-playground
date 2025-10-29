use std::error::Error;
use std::io;
use std::io::{BufRead, BufReader, Read, Stdin, Stdout, Write};
use std::net::TcpStream;
use crate::clear_screen;
use crate::table::{Align, Table, TableCell, TableRow};
use crate::table_menu::TableMenu;
use crate::http::{backend_socket_addr, read_toppings};
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
        // Nummernbasiert (1..=len)
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

        // let name = entry.split('#').next().unwrap_or(&entry).to_string();
        send_delete_topping(&name_to_delete)?;
        clear_screen(stdout).expect("Should clear screen.");
    
        //neu laden und anzeigen
        let body_after = read_toppings()?;
        list_toppings_from_str(stdout, &body_after)?;

        writeln!(stdout, "\x1b[1;31mEntfernt:\x1b[0m \x1b[1m{name}\x1b[0m", name = name_to_delete)?;
        stdout.flush()?;

    Ok(())
}

fn send_delete_topping(name: &str) -> io::Result<()> {
    use std::io::Write;
    use std::net::TcpStream;

    let name_enc = urlencoding::encode(name);
    let addr = backend_socket_addr()?;
    let mut stream =  TcpStream::connect(addr)?;

    let req = format!("DELETE /toppings?name={name_enc} HTTP/1.1\r\n
Host: {addr}\r\n
Connection: close\r\n
\r\n"
);

    stream.write_all(req.as_bytes())?;
    stream.flush()?;

    let mut reader = BufReader::new(stream);
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

    if (200..300).contains(&code) {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, format!("HTTP {}", code)))
    }
}

pub fn send_clear_toppings() -> io::Result<()> {
    let addr = backend_socket_addr()?;
    let mut stream =  TcpStream::connect(addr)?;

    write!(stream, "DELETE /toppigns/clear HTTP/1.1\r
Host: {addr}\r
Connection close\r
\r"
)?;

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
    let addr = backend_socket_addr()?;
    let mut stream =  TcpStream::connect(addr)?;
    let body_bytes = body.as_bytes();
    let body_length = body.as_bytes().len();

    let head = format!("POST {path} HTTP/1.1\r
Host: {addr}\r
Content-Type: text/plain; charset=utf-8\r
Content-Length: {body_length}\r
Connection: close\r
\r
");

    stream.write_all(head.as_bytes())?;
    stream.write_all(body_bytes)?;
    stream.flush()?;

    let mut resp = String::new();
    stream.read_to_string(&mut resp).ok();
    let code = resp.split_whitespace().nth(1).and_then(|s| s.parse::<u16>().ok()).unwrap_or(0);
    if !(200..300).contains(&code) {
        return Err(io::Error::new(io::ErrorKind::Other, format!("HTTP {}", code)));
    }

    Ok(())
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
