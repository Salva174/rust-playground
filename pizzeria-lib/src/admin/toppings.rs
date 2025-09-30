use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufWriter, BufRead, Stdin, Stdout, Write};
use crate::clear_screen;
use crate::table::{Align, Table, TableCell, TableRow};
use crate::table_menu::TableMenu;

pub fn edit_toppings(stdout: &mut Stdout, stdin: &Stdin) -> Result<(), Box<dyn std::error::Error>> {

    writeln!(stdout, "\x1b[1;31mToppings Editor\x1b[0m <Topping-Name> <Preis>: ")?;
    stdout.flush()?;

    let file_path = "toppings_text";

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    let mut writer = BufWriter::new(file);
    let mut first = true;

    loop {

        if first {
            write!(stdout, "Topping hinzufügen? (j/n)\nAktuelle Liste anzeigen? (t)\nListe löschen? (d)\n> ")?;
            first = false;
        } else {
            write!(stdout, "Weiteres Topping hinzufügen? (j/n)\nAktuelle Liste anzeigen? (t)\nListe löschen? (d)\n> ")?;
        }
        stdout.flush()?;

        let choice_raw = prompt(stdin, stdout, "")?;
        let choice = choice_raw.trim().to_lowercase();

        if choice == "n" || choice == "nein" || choice == "q" {
            clear_screen(stdout)?;
            writeln!(stdout, "Eingabe beendet.")?;
            return Ok(());
        }

        if choice == "d" || choice == "delete" {
            drop(writer);
            File::create(file_path)?;
            writer = BufWriter::new(OpenOptions::new()
                .create(true)
                .append(true)
                .open(file_path)?
            );
            clear_screen(stdout)?;
            writeln!(stdout, "\x1b[1;35mDatei geleert.\x1b[0m")?;
            first = true;
            continue;
        }

        if choice == "t" {
            clear_screen(stdout)?;
            list_toppings(file_path)?;
            continue;
        }

        if choice != "j" && choice != "ja" {
            writeln!(stdout, "Bitte 'j', 'n', 'd', 't' oder 'q' eingeben.")?;
            continue;
        }

        //Name abfragen
        let topping_name = {
            let input = prompt(stdin, stdout, "\x1b[4;34mName\x1b[0m: ")?;
            if input.eq_ignore_ascii_case("q") {
                clear_screen(stdout)?;
                writeln!(stdout, "Beende Eingabe.")?;
                return Ok(());
            }
            if input.trim().is_empty() {
                clear_screen(stdout)?;
                writeln!(stdout, "Beende Eingabe.")?;
                return Ok(());
            }
            input.trim().to_string()
        };

        //Preis abfragen
        let topping_price: u32 = loop {
            let input = prompt(stdin, stdout, "\x1b[4;34mPreis\x1b[0m (Ganzzahl): ")?;
            let input = input.trim();

            if input.eq_ignore_ascii_case("q") {
                clear_screen(stdout)?;
                writeln!(stdout, "Beende Eingabe.")?;
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

        //In Datei schreiben
        // let topping_name_no_whitespace = topping_name.replace(" ", "");
        writeln!(writer, "{}#{}", topping_name, topping_price)?;
        writer.flush()?;

        clear_screen(stdout)?;
        writeln!(stdout, "Erfolgreich hinzugefügt: \x1b[1;32m{} {}\x1b[0m", topping_name, topping_price)?;
    }
}

//Nutzereingabe lesen
fn prompt(stdin: &Stdin, stdout: &mut Stdout, label: &str) -> io::Result<String> {
    write!(stdout, "{}", label)?;
    stdout.flush()?;
    let mut buf = String::new();
    stdin.read_line(&mut buf)?;
    Ok(buf)
}

fn list_toppings(path: &str) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let title_text = String::from("Aktuelle Toppings");
    let mut table = Table::new( vec![]);

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('#').collect();
        if !parts.is_empty() {
            table.push(TableRow::new(vec![
                TableCell::new(format!("{}.", index + 1)),
                TableCell::new(parts[0].to_string()),
                TableCell::new_with_alignment(format!("{}.00$", parts[1]), Align::Right)
            ]));
        }
    }

    if table.is_empty() {
        let table = Table::new(vec![
            TableRow::new(vec![
                TableCell::new(String::from("Noch keine Toppings vorhanden!"))
            ])
        ]);
        let table_menu = TableMenu::new(title_text, table);
        println!("{table_menu}");
        return Ok(());
    }

    let table_menu = TableMenu::new(title_text, table);
    println!("{table_menu}");


    Ok(())
}

