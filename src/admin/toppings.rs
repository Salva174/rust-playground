use std::fs::OpenOptions;
use std::io;
use std::io::{BufWriter, Stdin, Stdout, Write};

pub fn edit_toppings(stdout: &mut Stdout, stdin: &Stdin) -> Result<(), Box<dyn std::error::Error>> {

    writeln!(stdout, "\x1b[1;30mToppings eingeben\x1b[0m <Topping-Name> <Preis>: ")?;
    writeln!(stdout, "Leere Zeile oder 'q' beendet.")?;
    stdout.flush()?;

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("toppings_text")?;

    let mut writer = BufWriter::new(file);

    loop {
        //Name abfragen
        let topping_name = {
            let input = prompt(stdin, stdout, "\x1b[4;30mName\x1b[0m: ")?;
            if input.eq_ignore_ascii_case("q") {
                writeln!(stdout, "Beende Eingabe.")?;
                return Ok(());
            }
            if input.trim().is_empty() {
                writeln!(stdout, "Beende Eingabe.")?;
                return Ok(());
            }
            input.trim().to_string()
        };

        //Preis abfragen
        let topping_price: u32 = loop {
            let input = prompt(stdin, stdout, "\x1b[4;30mPreis\x1b[0m (Ganzzahl): ")?;
            let input = input.trim();

            if input.eq_ignore_ascii_case("q") {
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
        let topping_name_no_whitespace = topping_name.replace(" ", "");
        writeln!(writer, "{} {}", topping_name_no_whitespace, topping_price)?;
        writer.flush()?;

        writeln!(stdout, "Erfolgreich hinzugefügt: \x1b[1;30m{} {}\x1b[0m", topping_name_no_whitespace, topping_price)?;
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

