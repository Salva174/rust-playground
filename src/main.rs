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
    writeln!(stdout, "C : Customize your own Pizza")?;

    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    match input.trim() {
        "1" => writeln!(stdout, "You ordered Margherita!")?,
        "2" => writeln!(stdout, "You ordered Salami!")?,
        "3" => writeln!(stdout, "You ordered Hawaii!")?,
        "4" => writeln!(stdout, "You ordered Pepperoni!")?,
        "C" | "c" => order_custom_pizza(stdout, stdin)?,
        &_ => writeln!(stdout, "Not in Menu.")?
    }
    Ok(())
}

fn order_custom_pizza(stdout: &mut Stdout, stdin: &Stdin) -> Result<(), Box<dyn std::error::Error>> {

    let mut input = String::new();
    let mut toppings = Vec::<String>::new();

    loop {

        writeln!(stdout, "Choose your toppings:")?;
        writeln!(stdout, "B: Bacon")?;
        writeln!(stdout, "O: Onions")?;
        writeln!(stdout, "M: Mushrooms")?;
        writeln!(stdout, "Q: Quit")?;

        input.clear();
        stdin.read_line(&mut input).unwrap();

        match input.trim() {
            "B" | "b" => {
                writeln!(stdout, "You added Bacon to your Pizza!")?;
                toppings.push(String::from("Bacon"));
            },
            "O" | "o" => {
                writeln!(stdout, "You added Onions to your Pizza!")?;
                toppings.push(String::from("Onions"));
            },
            "M" | "m" => {
                writeln!(stdout, "You added Mushrooms to your Pizza!")?;
                toppings.push(String::from("Mushrooms"));
            },
             &_ => {
                 writeln!(stdout, "Your toppings: {}", toppings.join(", "))?;
                 break;
             }
        }

    }

    Ok(())
}