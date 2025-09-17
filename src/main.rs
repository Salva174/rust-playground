use std::io::{Stdin, Stdout, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = std::io::stdout();
    let stdin= std::io::stdin();


    writeln!(stdout, "Welcome to Salvatores Pizza!")?;
    writeln!(stdout, "1: Order Pizza")?;
    writeln!(stdout, "2: Quit")?;

    write!(stdout, "> ")?;
    stdout.flush();

    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    match input.trim() {
        "1" => order_pizza(&mut stdout, &stdin)?,
        &_ => writeln!(stdout, "Quit")?
    }

    Ok(())
}

fn order_pizza(stdout: &mut Stdout, stdin: &Stdin) -> Result<(), Box<dyn std::error::Error>> {

    writeln!(stdout, "Pizza-Menu:")?;
    writeln!(stdout, "1: Pizza-Margherita")?;
    writeln!(stdout, "2: Pizza-Salami")?;
    writeln!(stdout, "3: Pizza-Hawaii")?;
    writeln!(stdout, "4: Pizza-Pepperoni")?;

    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    match input.trim() {
        "1" => writeln!(stdout, "You ordered Margherita!")?,
        "2" => writeln!(stdout, "You ordered Salami!")?,
        "3" => writeln!(stdout, "You ordered Hawaii!")?,
        "4" => writeln!(stdout, "You ordered Pepperoni!")?,
        &_ => writeln!(stdout, "Not in Menu.")?
    }
    Ok(())
}