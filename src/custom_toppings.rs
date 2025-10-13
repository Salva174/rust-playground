use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Stdin, Stdout, Write};
use pizzeria_lib::admin::toppings::list_toppings;
use pizzeria_lib::clear_screen;
use crate::ui::{wait_enter, prompt};

// Entfernen nach Nummer oder Name
pub fn remove_topping(stdout: &mut Stdout, stdin: &mut Stdin, path: &str) -> io::Result<()> {

    list_toppings(stdout, path)?;

    // Eingabe erfragen
    let choice = prompt(stdin, stdout, "\nEintrag löschen (Nummer oder Name, 'q' zum Abbrechen): ")?;
    let choice = choice.trim();
    if choice.eq_ignore_ascii_case("q") || choice.is_empty() {
        writeln!(stdout, "Abgebrochen.")?;
        return Ok(());
    }

    // Datei einlesen
    let file = File::open(path).or_else(|_| File::create(path))?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader
        .lines()
        .map_while(Result::ok)
        .filter(|l| !l.trim().is_empty())
        .collect();

    if lines.is_empty() {
        writeln!(stdout, "Keine Toppings vorhanden.")?;
        stdout.flush()?;
        wait_enter(stdout, stdin, "\n[Weiter mit Enter]")?;
        return Ok(());
    }

    // Auswahl interpretieren
    let removed = if let Ok(idx1) = choice.parse::<usize>() {
        // Nummernbasiert (1..=len)
        if idx1 == 0 || idx1 > lines.len() {
            writeln!(stdout, "Ungültige Nummer.")?;
            stdout.flush()?;
            wait_enter(stdout, stdin, "\n[Weiter mit Enter]")?;
            return Ok(());
        }
        let entry = lines.remove(idx1 - 1);
        Some(entry)
    } else {
        // Namensbasiert: suche ersten Eintrag vor '#'
        if let Some(pos) = lines.iter().position(|l| l.split('#').next().unwrap_or("").eq_ignore_ascii_case(choice)) {
            Some(lines.remove(pos))
        } else {
            writeln!(stdout, "Kein Eintrag mit diesem Namen gefunden.")?;
            stdout.flush()?;
            wait_enter(stdout, stdin, "\n[Weiter mit Enter]")?;
            None
        }
    };

    // Zurückschreiben, falls etwas entfernt wurde
    if let Some(entry) = removed {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        for l in &lines {
            writeln!(writer, "{l}")?;
        }
        writer.flush()?;

        let name = entry.split('#').next().unwrap_or(&entry);
        writeln!(stdout, "\x1b[1;31mEntfernt:\x1b[0m \x1b[1m{name}\x1b[0m")?;
    }

    Ok(())
}

pub fn add_toppings(stdout: &mut Stdout, stdin: &mut Stdin) -> Result<(), Box<dyn Error>> {
    let file_path = "toppings_text";

    // File-Writer einmal öffnen und für alle Adds nutzen
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;
    let mut writer = BufWriter::new(file);

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

        writeln!(writer, "{}#{}", topping_name, topping_price)?;
        writer.flush()?;

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