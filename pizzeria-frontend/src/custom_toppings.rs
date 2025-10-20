use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Read, Stdin, Stdout, Write};
use std::net::TcpStream;
use pizzeria_lib::admin::toppings::list_toppings;
use pizzeria_lib::clear_screen;
use crate::http::read_toppings;
use crate::ui::{wait_enter, prompt};

// Entfernen nach Nummer oder Name
pub fn remove_topping(stdout: &mut Stdout, stdin: &mut Stdin, _path: &str) -> io::Result<()> {

    let body = read_toppings()?; // GET /toppings (wie bei dir)
    let mut lines: Vec<String> = body
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();

    list_toppings_from_lines(stdout, &lines)?; // kleine Helper-Funktion, die statt Datei die lines rendert

    // Eingabe erfragen
    let choice = prompt(stdin, stdout, "\nEintrag löschen (Nummer oder Name, 'q' zum Abbrechen): ")?;
    let choice = choice.trim();
    if choice.eq_ignore_ascii_case("q") || choice.is_empty() {
        writeln!(stdout, "Abgebrochen.")?;
        return Ok(());
    }

    // // Datei einlesen
    // let file = File::open(path).or_else(|_| File::create(path))?;
    // let reader = BufReader::new(file);
    // let mut lines: Vec<String> = reader
    //     .lines()
    //     .map_while(Result::ok)
    //     .filter(|l| !l.trim().is_empty())
    //     .collect();

    if lines.is_empty() {
        writeln!(stdout, "Keine Toppings vorhanden.")?;
        stdout.flush()?;
        wait_enter(stdout, stdin, "\n[Weiter mit Enter]")?;
        return Ok(());
    }

    // Auswahl interpretieren
    let name_to_delete: String = if let Ok(idx1) = choice.parse::<usize>() {
        // Nummernbasiert (1..=len)
        if idx1 == 0 || idx1 > lines.len() {
            writeln!(stdout, "Ungültige Nummer.")?;
            stdout.flush()?;
            wait_enter(stdout, stdin, "\n[Weiter mit Enter]")?;
            return Ok(());
        }
        let entry = &lines[idx1 - 1];
        entry.split('#').next().unwrap_or("").to_string()
    } else {
        // Namensbasiert: suche ersten Eintrag vor '#'
        if let Some(pos) = lines.iter().position(|l| l.split('#').next().unwrap_or("").eq_ignore_ascii_case(choice)) {
            lines[pos].split('#').next().unwrap_or("").to_string()
        } else {
            writeln!(stdout, "Kein Eintrag mit diesem Namen gefunden.")?;
            stdout.flush()?;
            wait_enter(stdout, stdin, "\n[Weiter mit Enter]")?;
            return Ok(());
        }
    };

    // Zurückschreiben, falls etwas entfernt wurde
    // if let Some(entry) = removed {
        // let file = File::create(path)?;
        // let mut writer = BufWriter::new(file);
        // for l in &lines {
        //     writeln!(writer, "{l}")?;
        // }
        // writer.flush()?;

        // let name = entry.split('#').next().unwrap_or(&entry).to_string();
        send_delete_topping(&name_to_delete)?;
        writeln!(stdout, "\x1b[1;31mEntfernt:\x1b[0m \x1b[1m{name}\x1b[0m", name = name_to_delete)?;

    Ok(())
}

fn send_delete_topping(name: &str) -> io::Result<()> {
    use std::io::{Read, Write};
    use std::net::TcpStream;

    let mut stream = TcpStream::connect("127.0.0.1:3333")?;
    let req = format!("DELETE /toppings?name={} HTTP/1.1\r
Host: 127.0.0.1:3333\r
Connection: close\r
",
url_encode(name)
);
    stream.write_all(req.as_bytes())?;
    stream.flush()?;

    let mut resp = String::new();
    stream.read_to_string(&mut resp).ok();
    let code = resp.split_whitespace().nth(1).and_then(|s| s.parse::<u16>().ok()).unwrap_or(0);
    if !(200..300).contains(&code) {
        return Err(io::Error::new(io::ErrorKind::Other, format!("HTTP {}", code)));
    }
    Ok(())
}

fn url_encode(s: &str) -> String {
    s.replace(' ', "%20")
}



pub fn add_toppings(stdout: &mut Stdout, stdin: &mut Stdin) -> Result<(), Box<dyn Error>> {
    // let file_path = "toppings_text";
    //
    // // File-Writer einmal öffnen und für alle Adds nutzen
    // let file = OpenOptions::new()
    //     .create(true)
    //     .append(true)
    //     .open(file_path)?;
    // let mut writer = BufWriter::new(file);

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

        let mut line = format!("{}#{}", topping_name, topping_price);
        if !line.ends_with('\n') { line.push('\n'); }

        writeln!(stdout, "{}#{}", topping_name, topping_price)?;
        stdout.flush()?;

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
    let mut stream = TcpStream::connect("127.0.0.1:3333")?;
    let body_length = body.as_bytes().len();

    let req = format!("POST {path} HTTP/1.1\r
Host: 127.0.0.1:3333\r
Content-Type: text/plain; charset=utf-8\r
Content-Length: {body_length}\r
Connection: close\r
\r
{body}");

    stream.write_all(req.as_bytes())?;
    stream.flush()?;

    let mut resp = String::new();
    stream.read_to_string(&mut resp).ok();
    let code = resp.split_whitespace().nth(1).and_then(|s| s.parse::<u16>().ok()).unwrap_or(0);
    if !(200..300).contains(&code) {
        return Err(io::Error::new(io::ErrorKind::Other, format!("HTTP {}", code)));
    }

    Ok(())
}

fn list_toppings_from_lines(stdout: &mut Stdout, lines: &[String]) -> std::io::Result<()> {
    writeln!(stdout, "\n\x1b[1;33mAktuelle Toppings\x1b[0m:")?;
    for (i, l) in lines.iter().enumerate() {
        let (name, price) = l.split_once('#').unwrap_or((l.as_str(), ""));
        writeln!(stdout, "{:>2}: {:<20} {}", i + 1, name, price)?;
    }
    Ok(())
}
