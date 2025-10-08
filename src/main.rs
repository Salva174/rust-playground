use std::io::{Stdin, Stdout, Write};
use std::fs;
use pizzeria_lib::clear_screen;
use pizzeria_lib::table::{Table, TableCell, TableRow};
use pizzeria_lib::table_menu::TableMenu;
use pizzeria_lib::types::{parse_prebuild_pizza, parse_toppings, Pizza, Topping};

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

    let table = Table::new(vec! [
        TableRow::new( vec! [
            TableCell::new(String::from("1:")),
            TableCell::new(String::from("Order Pizza"))
        ]),
        TableRow::new( vec! [
            TableCell::new(String::from("2:")),
            TableCell::new(String::from("Edit Toppings"))
        ]),
        TableRow::new( vec! [
            TableCell::new(String::from("3:")),
            TableCell::new(String::from("Quit"))
        ])
    ]);

    let table_menu = TableMenu::new(title_text, table);
    println!("{table_menu}");

    write!(stdout, "> ")?;
    stdout.flush()?;

    clear_screen(&mut stdout)?;

    let mut input = String::new();
    stdin.read_line(&mut input)?;

    match input.trim() {
        "1" => order_pizza(&mut stdout, &stdin, &toppings, &pizzas)?,
        // "2" => edit_toppings(&mut stdout, &stdin)?,
        &_ => writeln!(stdout, "Quit")?
    }

    Ok(())
}

fn order_pizza(stdout: &mut Stdout, stdin: &Stdin, available_toppings: &[Topping], prebuild_pizzas: &[Pizza]) -> Result<(), Box<dyn std::error::Error>> {

    let title_text = String::from("Pizza-Menu");
    let mut table = Table::new(vec![]);
    for (index, pizza) in prebuild_pizzas.iter().enumerate() {
        let pizza = &pizza.name;
        table.push(TableRow::new(vec![
            TableCell::new(format!("{index}:")),
            TableCell::new(String::from(pizza))
        ]))
    }
    table.push(TableRow::new( vec! [
        TableCell::new(String::from("C:")),
        TableCell::new(String::from("Customize your own Pizza"))
    ]));

    let table_menu = TableMenu::new(title_text, table);
    println!("{table_menu}");

    write!(stdout, "> ")?;
    stdout.flush()?;


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
        let mut table = Table::new(vec![]);

        for topping in available_toppings.iter() {
            let shortname = topping.shortname();
            let name = &topping.name;
            let price = &topping.price;
            table.push(TableRow::new( vec![
                TableCell::new(format!("{shortname}:")),
                TableCell::new(name.to_string()),
                TableCell::new(format!("{price}.00$"))
            ]))
        }
        let table_menu = TableMenu::new(title_text, table);
        println!("{table_menu}");
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
            let mut topping_entries_table = Table::new(vec![]);

            for topping in pizza.toppings.iter() {
                topping_entries_table.push(TableRow::new(vec![
                    TableCell::new(topping.name.to_string()),
                    TableCell::new(String::from("=")),
                    TableCell::new(format!("{}.00$", topping.price))
                ]));
            }

            let table_menu = TableMenu::new(title_text, topping_entries_table);
            println!("{table_menu}");
            writeln!(stdout, "Your price: \x1b[4;30m{}.00$\x1b[0m", pizza.total_price())?;

            break;
        }

    }

    Ok(())
}

