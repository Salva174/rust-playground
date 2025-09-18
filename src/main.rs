use std::io::{Stdin, Stdout, Write};

struct Topping {
    name: String,
    price: u32
}

struct Pizza {
    name: String,
    toppings: Vec<Topping>
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
        Topping { name: String::from("Bacon"), price: 3 },
        Topping { name: String::from("Ham"), price: 4 },
    ];

    let pizzas = vec! [
        Pizza {
            name: String::from("Margherita"),
            toppings: Vec::new()
        },
        Pizza {
            name: String::from("Pepperoni"),
            toppings: vec! [
                Topping { name: String::from("Pepperoni"), price: 8 }
            ]
        },
        Pizza {
            name: String::from("Salami"),
            toppings: vec! [
                Topping { name: String::from("Salami"), price: 7 }
            ]
        },
        Pizza {
            name: String::from("Hawaii"),
            toppings: vec! [
                Topping { name: String::from("Pineapple"), price: 4},
                Topping { name: String::from("Ham"), price: 5}
            ]
        }
    ];

    writeln!(stdout, "Welcome to Salvatores Pizza!")?;
    writeln!(stdout, "1: Order Pizza")?;
    writeln!(stdout, "2: Quit")?;

    write!(stdout, "> ")?;
    stdout.flush()?;

    let mut input = String::new();
    stdin.read_line(&mut input)?;

    match input.trim() {
        "1" => order_pizza(&mut stdout, &stdin, &toppings, &pizzas)?,
        &_ => writeln!(stdout, "Quit")?
    }

    Ok(())
}

fn order_pizza(stdout: &mut Stdout, stdin: &Stdin, available_toppings: &Vec<Topping>, prebuild_pizzas: &Vec<Pizza>) -> Result<(), Box<dyn std::error::Error>> {

    writeln!(stdout, "Pizza-Menu:")?;
    for (index, pizza) in prebuild_pizzas.iter().enumerate() {
        writeln!(stdout, "{}: {}", index, &pizza.name)?;
    }
    writeln!(stdout, "C: Customize your own Pizza")?;

    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    match input.trim() {
        "C" | "c" => order_custom_pizza(stdout, stdin, available_toppings)?,
        value => {
            match value.parse::<usize>() {
                Ok(index) => {
                    match prebuild_pizzas.get(index) {
                        None => { writeln!(stdout, "unknown Menu-entry")?;}
                        Some(pizza) => {
                            let price: u32 = pizza.toppings.iter().map(| topping | topping.price).sum();
                            writeln!(stdout, "Your choice: {} for {}.00$", pizza.name, price)?;
                        }
                    }
                }
                Err(_) => { writeln!(stdout, "Not in Menu.")? }
            }

        }
    }
    Ok(())
}

fn order_custom_pizza(stdout: &mut Stdout, stdin: &Stdin, available_toppings: &Vec<Topping>) -> Result<(), Box<dyn std::error::Error>> {

    let mut input = String::new();
    let mut pizza = Pizza {
        name: String::from("Custom"),
        toppings: Vec::new()
    };

    loop {

        writeln!(stdout, "Choose your toppings:")?;
        writeln!(stdout, "B: Bacon")?;
        writeln!(stdout, "O: Onions")?;
        writeln!(stdout, "M: Mushrooms")?;
        writeln!(stdout, "H: Ham")?;
        writeln!(stdout, "Q: Quit")?;

        input.clear();
        stdin.read_line(&mut input).unwrap();

        match input.trim() {
            "B" | "b" => {
                writeln!(stdout, "You added Bacon to your Pizza!")?;
                let topping = available_toppings.iter()
                    .find(|topping| topping.name == "Bacon")
                    .expect("There must be bacon!");
                pizza.toppings.push(Clone::clone(topping));
            },
            "O" | "o" => {
                writeln!(stdout, "You added Onions to your Pizza!")?;
                let topping = available_toppings.iter()
                    .find(|topping| topping.name == "Onions")
                    .expect("There must be Onions!");
                pizza.toppings.push(Clone::clone(topping));
            },
            "M" | "m" => {
                writeln!(stdout, "You added Mushrooms to your Pizza!")?;
                let topping = available_toppings.iter()
                    .find(|topping| topping.name == "Mushrooms")
                    .expect("Dont eat the red ones!");
                pizza.toppings.push(Clone::clone(topping));
            },
            "H" | "h" => {
                writeln!(stdout, "You added Ham to your Pizza!")?;
                let topping = available_toppings.iter()
                    .find(|topping | topping.name=="Ham")
                    .expect("There must be Ham!");
                pizza.toppings.push(Clone::clone(topping));
            }
             &_ => {
                 writeln!(stdout, "Your toppings: {}", pizza.toppings.iter().map(|topping| topping.name.as_str()).collect::<Vec<&str>>().join(", "))?;
                 let price: u32 = pizza.toppings.iter()
                     .map(|topping| topping.price)
                     .sum();
                 writeln!(stdout, "Your price: {}.00$", price)?;
                 break;
             }
        }

    }

    Ok(())
}