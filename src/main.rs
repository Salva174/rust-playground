mod types;

use std::io::{Stdin, Stdout, Write};
use crate::types::{parse_prebuild_pizza, parse_toppings, Pizza, Topping};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = std::io::stdout();
    let stdin= std::io::stdin();

    let toppings_file_path = "pizza_toppings_text";
    let content = fs::read_to_string(toppings_file_path).expect("There has to be text!");
    let toppings = parse_toppings(&content)?;

    let prebuild_pizza_file_path ="pizza_prebuilds_text";
    let prebuild_content = fs::read_to_string(prebuild_pizza_file_path).expect("There has to be text!");
    let pizzas = parse_prebuild_pizza(&prebuild_content, &toppings)?;
    
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
                            let price: u32 = pizza.total_price();
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
        toppings: Vec::new(),
        base_price: 8,
    };

    loop {

        writeln!(stdout, "Choose your toppings:")?;

        for topping in available_toppings.iter() {
            let shortname = topping.shortname();
            let name = &topping.name;
            writeln!(stdout, "{shortname}: {name}")?;
        }

        writeln!(stdout, "Q: Quit")?;

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let input= input.trim().chars().next().expect("Input must not be empty!");

        let mut selected_topping: Option<Topping> = None;
        for topping in available_toppings.iter() {
            let shortname_uppercase = topping.shortname();
            let shortname_lowercase = shortname_uppercase.to_lowercase().next().unwrap();
            if shortname_uppercase == input || shortname_lowercase == input {
                selected_topping = Some(Clone::clone(topping));
                break;
            }
        }

        if let Some(topping) = selected_topping {
            pizza.toppings.push(topping);
        }
        else {
            writeln!(stdout, "Your toppings: {}", pizza.toppings.iter().map(|topping| topping.name.as_str()).collect::<Vec<&str>>().join(", "))?;
            let price: u32 = pizza.total_price();
            writeln!(stdout, "Your price: {}.00$", price)?;
            break;
        }

    }

    Ok(())
}

