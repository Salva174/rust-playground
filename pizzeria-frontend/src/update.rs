use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{Stdin, Stdout, Write};
use pizzeria_lib::admin::toppings::list_toppings;
use pizzeria_lib::clear_screen;
use pizzeria_lib::table::{Table, TableCell, TableRow};
use pizzeria_lib::table::Align::Right;
use pizzeria_lib::table_menu::TableMenu;
use pizzeria_lib::types::{Pizza, Topping};
use crate::custom_toppings::{add_toppings, remove_topping};
use crate::http::send_transaction_record;
use crate::input::{read_input, InputEvent};
use crate::render::render_menu;
use crate::state::{MenuIndex, State};
use crate::transactions::{log_custom_pizza, log_custom_pizza_as_string, log_transaction, log_transaction_as_string};
use crate::ui::{confirm, wait_enter};

const LOG_PATH: &str = "transactions.log";

pub fn update(input: InputEvent, state: &mut State, stdout: &mut Stdout, stdin: &mut Stdin) -> bool {

    if let InputEvent::Exit = input {
        return true;
    }

    match state.current_menu {
        MenuIndex::MainMenu => main_menu_update(input, state),
        MenuIndex::EditToppingsMenu => edit_toppings_menu_update(input, state, stdout, stdin),
        MenuIndex::OrderMenu => order_menu_update(input, state, stdout, stdin),
    }
}

fn order_menu_update(input: InputEvent, state: &mut State, stdout: &mut Stdout, stdin: &mut Stdin) -> bool {

    match input {
        InputEvent::Up => {
            let length = state.current_menu_mut().table_mut().rows_mut().len();
            let sel_row = state.selected_row();
            if sel_row > 0 {
                state.set_selected_row(sel_row -1) ;
            } else {
                state.set_selected_row(length.saturating_sub(1));
            }
        }
        InputEvent::Down => {
            let length = state.current_menu_mut().table_mut().rows_mut().len();
            let sel_row = state.selected_row();
            if sel_row + 1 < length {
                state.set_selected_row(sel_row + 1);
            } else {
                state.set_selected_row(0);
            }
        }
        InputEvent::Back => {
            state.current_menu = MenuIndex::MainMenu;
            state.apply_selection_marker();
            return false;
        }
        InputEvent::Enter => {
            let sel_row = state.selected_row();
            let length = state.prebuilt_pizzas.len();
            let custom_row = length; // letzte Zeile ist Custom (nach n Pizzen eingefügt)

            if sel_row == custom_row {
                let base_price = 6;
                if let Err(e) = order_custom_pizza(stdout, stdin, &state.toppings_catalog, base_price) {
                    writeln!(stdout, "Fehler im Custom-Pizza-Dialog: {e}.").ok();
                    wait_enter(stdout, stdin, "\n[Weiter mit Enter]").ok();
                }
            } else if let Some(p) = state.prebuilt_pizzas.get(sel_row) {
                writeln!(stdout, "\n\x1b[4;32mBestellung bestätigt\x1b[0m: \x1b[1m{}\x1b[0m ({}.00$).", p.name, p.total_price()).ok();
                let price_cents = p.total_price() * 100;
                let transaction_string = log_transaction_as_string(price_cents, &p.name);
                if let Err(e) = send_transaction_record(transaction_string) {
                    writeln!(stdout, "Warnung: Konnte Transaktion nicht loggen: {e}").ok();
                } else if let Err(e) = log_transaction(LOG_PATH, price_cents, &p.name) {
                    writeln!(stdout, "Warnung: Konnte Transaktion nicht in datei loggen: {e}").ok();
                }
                wait_enter(stdout, stdin, "\n[OK mit Enter]").ok();
            } else {
                writeln!(stdout, "Ungültige Auswahl.").ok();
                wait_enter(stdout, stdin, "\n[Weiter mit Enter]").ok();
            }
        }
        _ => {}
    }

   state.apply_selection_marker();

    false
}

fn main_menu_update(input: InputEvent, state: &mut State) -> bool {

    match input {
        InputEvent::Up => {
            let length = state.current_menu_mut().table_mut().rows_mut().len();
            let sel_row = state.selected_row();
            if sel_row > 0 {
                state.set_selected_row(sel_row -1) ;
            } else {
                state.set_selected_row(length.saturating_sub(1));
            }
        }
        InputEvent::Down => {
            let length = state.current_menu_mut().table_mut().rows_mut().len();
            let sel_row = state.selected_row();
            if sel_row + 1 < length {
                state.set_selected_row(sel_row + 1);
            } else {
                state.set_selected_row(0);
            }
        }
        InputEvent::Enter => {
            match state.selected_row() {
                0 => {
                    state.refresh_order_menu();
                    state.current_menu = MenuIndex::OrderMenu;
                    state.apply_selection_marker();
                },
                1 => {
                    state.current_menu = MenuIndex::EditToppingsMenu;
                    state.apply_selection_marker();
                },
                2 => return true,
                _ => todo!()
            }
        }
        InputEvent::Back => {
            // nothing to do.
        }
        _ => {}
    }

   state.apply_selection_marker();

    false
}

fn edit_toppings_menu_update(input: InputEvent, state: &mut State, stdout: &mut Stdout, stdin: &mut Stdin) -> bool {

    match input {
        InputEvent::Up => {
            let length = state.current_menu_mut().table_mut().rows_mut().len();
            let sel_row = state.selected_row();
            if sel_row > 0 {
                state.set_selected_row(sel_row -1) ;
            } else {
                state.set_selected_row(length.saturating_sub(1));
            }
        }
        InputEvent::Down => {
            let length = state.current_menu_mut().table_mut().rows_mut().len();
            let sel_row = state.selected_row();
            if sel_row + 1 < length {
                state.set_selected_row(sel_row + 1);
            } else {
                state.set_selected_row(0);
            }
        }
        InputEvent::Enter => {
            let file_path = "toppings_text";
            match state.selected_row() {
                0 => {
                    let _ = clear_screen(stdout);
                    if let Err(e) = add_toppings(stdout, stdin) {
                        writeln!(stdout, "Fehler: {e}").ok();
                    }
                }
                1 => {
                    let _ = clear_screen(stdout);
                    if let Err(e) = remove_topping(stdout, stdin, file_path) {
                        writeln!(stdout, "Fehler {e}").ok();
                    }
                    wait_enter(stdout, stdin, "\n[Weiter mit Enter]").ok();
                }
                2 => {
                    let _ = clear_screen(stdout);
                    if let Err(e) = list_toppings(stdout, file_path) {
                        writeln!(stdout, "Fehler {e}").ok();
                    }
                    wait_enter(stdout, stdin, "\n[Weiter mit Enter]").ok();
                }
                3 => {
                    // let _ = clear_screen(stdout);

                    match confirm(stdin, stdout, "\n\x1b[34mListe wirklich löschen?\x1b[0m (\x1b[32mY\x1b[0m/\x1b[31mN\x1b[0m): ") {
                        Ok(true) => {
                            if let Err(e) = clear_toppings_file(file_path) {
                                writeln!(stdout, "Fehler: {e}").ok();
                            } else {
                                writeln!(stdout, "\x1b[1;35mDatei geleert. \x1b[0m").ok();
                            }
                        }
                        Ok(false) => {
                            writeln!(stdout, "\nAbgebrochen - Liste wurde nicht gelöscht.").ok();
                        }
                        Err(e) => {
                            writeln!(stdout, "\nFehler bei der Eingabe: {e}").ok();
                        }
                    }
                    wait_enter(stdout, stdin, "\n[Weiter mit Enter]").ok();
                }
                _ => {}
            }
        }
        InputEvent::Back => {
            state.current_menu = MenuIndex::MainMenu;
        }
        _ => {}
    }

    state.apply_selection_marker();

    false
}

pub fn select_row(table: &mut Table, selected_row: usize) {
    for (index, row) in table.rows_mut().iter_mut().enumerate() {
        let cell = &mut row.cells_mut()[0];
        cell.text_mut().clear();
        if index == selected_row {
            cell.text_mut().push('>');
        } else {
            cell.text_mut().push(' ');
        }
    }
}

fn clear_toppings_file(path: &str) -> io::Result<()> {
    File::create(path).map(|_| ())
}

pub fn order_custom_pizza(stdout: &mut Stdout, stdin:  &mut Stdin, available_toppings: &[Topping], base_price: u32, ) -> Result<(), Box<dyn Error>> {
    let mut selected_row: usize = 0;
    let n = available_toppings.len();
    let checkout_row = n;
    let clear_row    = n + 1;
    let back_row     = n + 2;

    // Menge je Topping (für Mehrfachauswahl)
    let mut qty = vec![0u32; n];
    let mut in_buf = [0u8; 64];

    loop {
        // Render
        clear_screen(stdout)?;
        let mut table = Table::new(vec![]);

        for (i, t) in available_toppings.iter().enumerate() {
            let marker = if i == selected_row { ">" } else { " " };
            let qty_str = if qty[i] > 0 { format!("x{}", qty[i]) } else { String::new() };

            table.push(TableRow::new(vec![
                TableCell::new(marker.into()),
                TableCell::new(format!("{}:", i + 1)),
                TableCell::new(t.name.clone()),
                TableCell::new_with_alignment(format!("{}.00$", t.price), Right),
                TableCell::new_with_alignment(format!(" {qty_str}"), Right),
            ]));
        }

        table.push(TableRow::new(vec![
            TableCell::new(" ".into()),
            TableCell::new(String::new()),
            TableCell::new(String::new()),
            TableCell::new_with_alignment(String::new(), Right),
            TableCell::new_with_alignment(String::new(), Right),
        ]));

        // Aktionen
        let make_action = |idx: usize, tag: &str, label: &str| {
            TableRow::new(vec![
                TableCell::new(if selected_row == idx { ">" } else { " " }.into()), // Marker
                TableCell::new(tag.into()),                                         // Icon/Index
                TableCell::new(label.into()),                                       // Text
                TableCell::new_with_alignment(String::new(), Right),         // Preis-Platzhalter
                TableCell::new_with_alignment(String::new(), Right),         // Menge-Platzhalter
            ])
        };

        // Aktionen (mit 5 Spalten!)
        table.push(make_action(checkout_row, "C", "Checkout"));
        table.push(make_action(clear_row,    "CL", "Clear selection"));
        table.push(make_action(back_row,     "B", "Back"));

        // Menütitel + Ausgabe
        let tm = TableMenu::new("Custom Pizza".into(), table);

        let toppings_sum: u32 = qty.iter().enumerate().map(|(i, &q)| q * available_toppings[i].price).sum();
        let total = base_price + toppings_sum;

        let footer = [
            "",
            &format!("Basispreis: {}.00$ | Toppings: {}.00$ | Gesamt: \x1b[1m{}.00$\x1b[0m",
                        base_price, toppings_sum, total),
            "[↑/↓] bewegen · [Enter] hinzufügen/auswählen · [←] entfernen · [Backspace] zurück",
        ];
        render_menu(stdout, &tm, "CustomPizza", selected_row, &footer)?;
        stdout.flush()?;

        // Eingabe
        let ev = read_input(stdin, &mut in_buf)?;
        match ev {
            InputEvent::Up => {
                if selected_row > 0 { selected_row -= 1; } else { selected_row = back_row; }
            }
            InputEvent::Down => {
                if selected_row < back_row { selected_row += 1; } else { selected_row = 0; }
            }
            InputEvent::Left => {
                if selected_row < n && qty[selected_row] > 0 {
                    qty[selected_row] -= 1;
                }
            }
            InputEvent::Back => {
                // Abbruch zurück zum Order-Menü
                return Ok(());
            }
            InputEvent::Enter => {
                if selected_row < n {
                    // topping hinzufügen
                    qty[selected_row] += 1;
                } else if selected_row == checkout_row {
                    // Checkout: Zusammenfassung + Preis anzeigen
                    clear_screen(stdout)?;
                    let mut sum_table = Table::new(vec![]);
                    for (i, &q) in qty.iter().enumerate().filter(|(_, q)| **q > 0) {
                        sum_table.push(TableRow::new(vec![
                            TableCell::new(format!("{} x {}", available_toppings[i].name, q)),
                            TableCell::new_with_alignment(format!("{}.00$", available_toppings[i].price * q), Right),
                        ]));
                    }
                    let tm2 = TableMenu::new("Your toppings".into(), sum_table);
                    writeln!(stdout, "{tm2}")?;
                    let pizza = Pizza {
                        name: "Custom".into(),
                        base_price,
                        toppings: {
                            let mut v = Vec::new();
                            for (i, &q) in qty.iter().enumerate() {
                                for _ in 0..q { v.push(available_toppings[i].clone()); }
                            }
                            v
                        },
                    };
                    writeln!(stdout, "Gesamtpreis: \x1b[4;30m{}.00$\x1b[0m", pizza.total_price())?;
                    let transaction_string = log_custom_pizza_as_string(base_price, available_toppings, &qty, true);
                    if let Err(e) = log_custom_pizza(LOG_PATH, base_price, available_toppings, &qty, true) {
                        writeln!(stdout, "Warnung: Konnte Transaktion nicht loggen: {e}").ok();
                    } else if let Err(e) = send_transaction_record(transaction_string) {
                        writeln!(stdout, "Warnung: Konnte Transaktion nicht loggen: {e}").ok();
                    }
                    wait_enter(stdout, stdin, "\n[OK mit Enter]")?;
                    return Ok(());
                } else if selected_row == clear_row {
                    // Auswahl zurücksetzen
                    for q in &mut qty { *q = 0; }
                } else {
                    // Back
                    return Ok(());
                }
            }
            _ => {}
        }
    }
}
