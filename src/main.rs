mod types;
mod console;
mod admin;

use std::io::{Stdin, Stdout, Write};
use crate::types::{parse_prebuild_pizza, parse_toppings, Pizza, Topping};
use std::fs;
use crate::admin::toppings::edit_toppings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = std::io::stdout();
    let stdin= std::io::stdin();

    let toppings_file_path = "pizza_toppings_text";
    let content = fs::read_to_string(toppings_file_path).expect("There has to be text!");
    let toppings = parse_toppings(&content)?;

    let prebuild_pizza_file_path ="pizza_prebuilds_text";
    let prebuild_content = fs::read_to_string(prebuild_pizza_file_path).expect("There has to be text!");
    let pizzas = parse_prebuild_pizza(&prebuild_content, &toppings)?;

    clear_screen(&mut stdout)?;

    let title_text = String::from("Welcome to Salvatores Pizza!");

    let panel = console::Menu::new(title_text, vec![
        String::from("1: Order Pizza"),
        String::from("2: Edit Toppings"),
        String::from("3: Quit"),
    ]);
    writeln!(stdout, "{panel}")?;


    write!(stdout, "> ")?;
    stdout.flush()?;

    clear_screen(&mut stdout)?;

    let mut input = String::new();
    stdin.read_line(&mut input)?;

    match input.trim() {
        "1" => order_pizza(&mut stdout, &stdin, &toppings, &pizzas)?,
        "2" => edit_toppings(&mut stdout, &stdin)?,
        &_ => writeln!(stdout, "Quit")?
    }

    Ok(())
}

fn order_pizza(stdout: &mut Stdout, stdin: &Stdin, available_toppings: &[Topping], prebuild_pizzas: &[Pizza]) -> Result<(), Box<dyn std::error::Error>> {

    let title_text = String::from("Pizza-Menu");
    let mut menu_entries = Vec::<String>::new();
    for (index, pizza) in prebuild_pizzas.iter().enumerate() {
        let pizza = &pizza.name;
        menu_entries.push(format!("{index}: {pizza}"))
    }
    menu_entries.push(String::from("C: Customize your own Pizza"));
    let panel = console::Menu::new(title_text, menu_entries);
    writeln!(stdout, "{panel}")?;

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
                            writeln!(stdout, "Your choice: \x1b[1;30m{}\x1b[0m for \x1b[1;30m{}.00$\x1b[0m.", pizza.name, price)?;
                        }
                    }
                }
                Err(_) => { writeln!(stdout, "Not in Menu.")? }
            }

        }
    }
    Ok(())
}

fn order_custom_pizza(stdout: &mut Stdout, stdin: &Stdin, available_toppings: &[Topping]) -> Result<(), Box<dyn std::error::Error>> {

    let mut input = String::new();
    let mut pizza = Pizza {
        name: String::from("Custom"),
        toppings: Vec::new(),
        base_price: 8,
    };

    loop {
        clear_screen(stdout)?;

        let title_text = String::from("Choose your toppings");
        let mut menu_entries = Vec::<String>::new();

        for topping in available_toppings.iter() {
            let shortname = topping.shortname();
            let name = &topping.name;
            menu_entries.push(format!("{shortname}: {name}"))
        }
        let panel = console::Menu::new(title_text, menu_entries);
        writeln!(stdout, "{panel}")?;

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
            clear_screen(stdout)?;
            let title_text = String::from("Your toppings");
            let mut topping_entries = Vec::<String>::new();

            for topping in pizza.toppings.iter() {
                topping_entries.push(topping.name.to_string())
            }

            let panel = console::Menu::new(title_text, topping_entries);
            writeln!(stdout, "{panel}")?;
            writeln!(stdout, "Your price: {}.00$", pizza.total_price())?;

            break;
        }

    }

    Ok(())
}

fn clear_screen(stdout: &mut Stdout) -> Result<(), Box<dyn std::error::Error>> {
    write!(stdout, "\x1B[2J\x1B[1;1H")?;
    Ok(())
}