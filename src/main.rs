use std::io::{Stdin, Stdout, Write};
use std::io::ErrorKind::AddrNotAvailable;
use std::thread::available_parallelism;

struct Topping {
    name: String,
    price: u32
}

impl Clone for Topping {
    fn clone(&self) -> Self {
        Topping {
            name: Clone::clone(&self.name),
            price: self.price
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = std::io::stdout();
    let stdin= std::io::stdin();

    let toppings = vec! [
        Topping { name: String::from("Mushrooms"), price: 1 },
        Topping { name: String::from("Onions"), price: 2 },
        Topping { name: String::from("Bacon"), price: 3 }
    ];

    writeln!(stdout, "Welcome to Salvatores Pizza!")?;
    writeln!(stdout, "1: Order Pizza")?;
    writeln!(stdout, "2: Quit")?;

    write!(stdout, "> ")?;
    stdout.flush();

    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    match input.trim() {
        "1" => order_pizza(&mut stdout, &stdin, &toppings)?,
        &_ => writeln!(stdout, "Quit")?
    }

    Ok(())
}

fn order_pizza(stdout: &mut Stdout, stdin: &Stdin, available_toppings: &Vec<Topping>) -> Result<(), Box<dyn std::error::Error>> {

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
        "C" | "c" => order_custom_pizza(stdout, stdin, available_toppings)?,
        &_ => writeln!(stdout, "Not in Menu.")?
    }
    Ok(())
}

fn order_custom_pizza(stdout: &mut Stdout, stdin: &Stdin, available_toppings: &Vec<Topping>) -> Result<(), Box<dyn std::error::Error>> {

    let mut input = String::new();
    let mut toppings = Vec::<Topping>::new();

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
                let topping = available_toppings.iter()
                    .find(|topping| topping.name == "Bacon")
                    .expect("There must be bacon!");
                toppings.push(Clone::clone(topping));
            },
            "O" | "o" => {
                writeln!(stdout, "You added Onions to your Pizza!")?;
                let topping = available_toppings.iter()
                    .find(|topping| topping.name == "Onions")
                    .expect("There must be Onions!");
                toppings.push(Clone::clone(topping));
            },
            "M" | "m" => {
                writeln!(stdout, "You added Mushrooms to your Pizza!")?;
                let topping = available_toppings.iter()
                    .find(|topping| topping.name == "Mushrooms")
                    .expect("Dont eat the red ones!");
                toppings.push(Clone::clone(topping));
            },
             &_ => {
                 writeln!(stdout, "Your toppings: {}", toppings.iter().map(|topping| topping.name.as_str()).collect::<Vec<&str>>().join(", "))?;
                 let price: u32 = toppings.iter().map(|topping| topping.price).sum();
                 writeln!(stdout, "Your price: {}", price)?;
                 break;
             }
        }

    }

    Ok(())
}