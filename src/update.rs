use std::error::Error;
use std::fs::{File, OpenOptions};
use std::{fs, io};
use std::io::{BufRead, BufReader, BufWriter, Read, Stdin, Stdout, Write};
use pizzeria_lib::admin::toppings::list_toppings;
use pizzeria_lib::clear_screen;
use pizzeria_lib::table::{Table, TableCell, TableRow};
use pizzeria_lib::table::Align::Right;
use pizzeria_lib::table_menu::TableMenu;
use pizzeria_lib::types::{load_toppings_from_file, parse_prebuild_pizza, Pizza, Topping};
use crate::input::InputEvent;
use crate::state::{MenuIndex, State};

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
            if state.selected_row > 0 {
                state.selected_row -= 1;
            } else {
                state.selected_row = state.current_menu_mut().table_mut().rows_mut().len() - 1;
            }
        }
        InputEvent::Down => {
            if state.selected_row < state.current_menu_mut().table_mut().rows_mut().len() - 1 {
                state.selected_row += 1;
            } else {
                state.selected_row = 0;
            }
        }
        InputEvent::Back => {
            state.current_menu = MenuIndex::MainMenu;
        }
        InputEvent::Enter => {
            let len = state.prebuilt_pizzas.len();
            let custom_row = len; // letzte Zeile ist Custom (nach n Pizzen eingefügt)

            if state.selected_row == custom_row {
                // TODO: Custom Pizza Flow (später)
                writeln!(stdout, "Custom Pizza-Konfigurator coming soon.").ok();
                wait_enter(stdout, stdin, "\n[Weiter mit Enter]").ok();
            } else if let Some(p) = state.prebuilt_pizzas.get(state.selected_row) {
                writeln!(stdout, "\n\x1b[4;32mBestellung bestätigt\x1b[0m: \x1b[1m{}\x1b[0m ({}.00$).", p.name, p.total_price()).ok();
                wait_enter(stdout, stdin, "\n[OK mit Enter]").ok();
            } else {
                writeln!(stdout, "Ungültige Auswahl.").ok();
                wait_enter(stdout, stdin, "\n[Weiter mit Enter]").ok();
            }
        }
        _ => {}
    }

    let selected_row = state.selected_row;

    select_row(state.current_menu_mut().table_mut(), selected_row);

    false
}

fn main_menu_update(input: InputEvent, state: &mut State) -> bool {

    match input {
        InputEvent::Up => {
            if state.selected_row > 0 {
                state.selected_row -= 1;
            } else {
                state.selected_row = state.current_menu_mut().table_mut().rows_mut().len() - 1;
            }
        }
        InputEvent::Down => {
            if state.selected_row < state.current_menu_mut().table_mut().rows_mut().len() - 1 {
                state.selected_row += 1;
            } else {
                state.selected_row = 0;
            }
        }
        InputEvent::Enter => {
            match state.selected_row {
                0 => {
                    state.refresh_order_menu();
                    state.current_menu = MenuIndex::OrderMenu;
                },
                1 => state.current_menu = MenuIndex::EditToppingsMenu,
                2 => return true,
                _ => todo!()
            }
        }
        InputEvent::Back => {
            // nothing to do.
        }
        _ => {}
    }

    let selected_row = state.selected_row;

    select_row(state.current_menu_mut().table_mut(), selected_row);

    false
}

fn edit_toppings_menu_update(input: InputEvent, state: &mut State, stdout: &mut Stdout, stdin: &mut Stdin) -> bool {

    match input {
        InputEvent::Up => {
            if state.selected_row > 0 {
                state.selected_row -= 1;
            } else {
                state.selected_row = state.current_menu_mut().table_mut().rows_mut().len() - 1;
            }
        }
        InputEvent::Down => {
            if state.selected_row < state.current_menu_mut().table_mut().rows_mut().len() - 1 {
                state.selected_row += 1;
            } else {
                state.selected_row = 0;
            }
        }
        InputEvent::Enter => {
            let file_path = "toppings_text";
            match state.selected_row {
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
                    if let Err(e) = clear_toppings_file(file_path) {
                        writeln!(stdout, "Fehler: {e}").ok();
                    } else {
                        writeln!(stdout, "\x1b[1;35mDatei geleert. \x1b[0m").ok();
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

    let selected_row = state.selected_row;

    select_row(state.current_menu_mut().table_mut(), selected_row);

    false
}

fn select_row(table: &mut Table, selected_row: usize) {
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
    // truncate durch create() ohne append
    File::create(path).map(|_| ())
}

fn wait_enter(stdout: &mut Stdout, stdin: &mut Stdin, msg: &str) -> io::Result<()> {
    write!(stdout, "{msg}")?;
    stdout.flush()?;

    let mut b = [0u8; 1];
    loop {
        let n = stdin.read(&mut b)?;
        if n == 0 { break; }
        if b[0] == b'\r' || b[0] == b'\n' { break; }
    }
    Ok(())
}

fn prompt(stdin: &mut Stdin, stdout: &mut Stdout, label: &str) -> io::Result<String> {
    write!(stdout, "{}", label)?;
    stdout.flush()?;

    let mut buf = Vec::new();
    let mut byte = [0u8; 1];

    loop {
        let n = stdin.read(&mut byte)?;
        if n == 0 { break; }
        match byte[0] {
            b'\r' | b'\n' => break,
            8 | 127 => { // Backspace
                if !buf.is_empty() {
                    buf.pop();
                    write!(stdout, "\x08 \x08")?;
                    stdout.flush()?;
                }
            }
            b => {
                buf.push(b);
                stdout.write_all(&[b])?;
                stdout.flush()?;
            }
        }
    }
    writeln!(stdout)?;
    Ok(String::from_utf8_lossy(&buf).trim().to_string())
}


// Entfernen nach Nummer oder Name (Nutzer gibt String ein)
fn remove_topping(stdout: &mut Stdout, stdin: &mut Stdin, path: &str) -> io::Result<()> {

    list_toppings(stdout, path)?;

    // Eingabe erfragen
    let choice = prompt(stdin, stdout, "\nEintrag löschen (Nummer oder Name, 'q' zum Abbrechen): ")?;
    let choice = choice.trim();
    if choice.eq_ignore_ascii_case("q") || choice.is_empty() {
        writeln!(stdout, "Abgebrochen.")?;
        return Ok(());
    }

    // Datei einlesen
    let file = File::open(path).or_else(|_| File::create(path))?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader
        .lines()
        .filter_map(Result::ok)
        .filter(|l| !l.trim().is_empty())
        .collect();

    if lines.is_empty() {
        writeln!(stdout, "Keine Toppings vorhanden.")?;
        stdout.flush()?;
        wait_enter(stdout, stdin, "\n[Weiter mit Enter]")?;
        return Ok(());
    }

    // Auswahl interpretieren
    let removed = if let Ok(idx1) = choice.parse::<usize>() {
        // Nummernbasiert (1..=len)
        if idx1 == 0 || idx1 > lines.len() {
            writeln!(stdout, "Ungültige Nummer.")?;
            stdout.flush()?;
            wait_enter(stdout, stdin, "\n[Weiter mit Enter]")?;
            return Ok(());
        }
        let entry = lines.remove(idx1 - 1);
        Some(entry)
    } else {
        // Namensbasiert: suche ersten Eintrag vor '#'
        if let Some(pos) = lines.iter().position(|l| l.split('#').next().unwrap_or("").eq_ignore_ascii_case(choice)) {
            Some(lines.remove(pos))
        } else {
            writeln!(stdout, "Kein Eintrag mit diesem Namen gefunden.")?;
            stdout.flush()?;
            wait_enter(stdout, stdin, "\n[Weiter mit Enter]")?;
            None
        }
    };

    // Zurückschreiben, falls etwas entfernt wurde
    if let Some(entry) = removed {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        for l in &lines {
            writeln!(writer, "{l}")?;
        }
        writer.flush()?;

        let name = entry.split('#').next().unwrap_or(&entry);
        writeln!(stdout, "\x1b[1;31mEntfernt:\x1b[0m \x1b[1m{name}\x1b[0m")?;
    }

    Ok(())
}

pub fn add_toppings(stdout: &mut Stdout, stdin: &mut Stdin) -> Result<(), Box<dyn Error>> {
    let file_path = "toppings_text";

    // File-Writer einmal öffnen und für alle Adds nutzen
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;
    let mut writer = BufWriter::new(file);

    loop {
        clear_screen(stdout)?;
        writeln!(stdout, "\x1b[1;31mTopping hinzufügen\x1b[0m (Name, dann Preis). 'q' zum Abbrechen.")?;
        stdout.flush()?;

        //  Name
        let topping_name = {
            let input = prompt(stdin, stdout, "\x1b[4;34mName\x1b[0m: ")?;
            let name = input.trim();
            if name.is_empty() || name.eq_ignore_ascii_case("q") {
                writeln!(stdout, "Abgebrochen.")?;
                return Ok(());
            }
            name.to_string()
        };

        //  Preis
        let topping_price: u32 = loop {
            let input = prompt(stdin, stdout, "\x1b[4;34mPreis (Ganzzahl)\x1b[0m: ")?;
            let input = input.trim();

            if input.eq_ignore_ascii_case("q") || input.is_empty() {
                writeln!(stdout, "Abgebrochen.")?;
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

        writeln!(writer, "{}#{}", topping_name, topping_price)?;
        writer.flush()?;

        writeln!(stdout, "\nErfolgreich hinzugefügt: \x1b[1;32m{} {}\x1b[0m", topping_name, topping_price)?;
        stdout.flush()?;

        let again = prompt(stdin, stdout, "Weiteres Topping hinzufügen? (j/n): ")?;
        let again = again.trim().to_lowercase();
        if again != "j" && again != "ja" {
            break;
        }
    }

    Ok(())
}


pub fn load_prebuilt_pizzas_from_file(path: &str, available:  &[Topping]) -> io::Result<Vec<Pizza>> {
    let content = fs::read_to_string(path)?;
    parse_prebuild_pizza(&content, available).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

pub fn build_order_menu(prebuilt: &[Pizza]) -> TableMenu {
    let mut table = Table::new(vec![]);

    if prebuilt.is_empty() {
        table.push(TableRow::new(vec![
            TableCell::new(" ".into()),
            TableCell::new("-".into()),
            TableCell::new("Keine Prebuilt-Pizzen vorhanden".into()),
        ]));
    } else {
        for (i, p) in prebuilt.iter().enumerate() {
            table.push(TableRow::new(vec![
                TableCell::new(" ".into()),
                TableCell::new(format!("{}:", i + 1)),
                TableCell::new(format!("{}", p.name)),
                TableCell::new_with_alignment(format!("{}.00$", p.total_price()), Right),
            ]));
        }
    }

    // Letzte Zeile: Custom Pizza
    table.push(TableRow::new(vec![
        TableCell::new(" ".into()),
        TableCell::new("C:".into()),
        TableCell::new("Custom Pizza".into()),
    ]));

    TableMenu::new("Order Menu".into(), table)
}

pub fn build_order_menu_error(err_msg: &str) -> TableMenu {
    let table = Table::new(vec![
        TableRow::new(vec![
            TableCell::new(" ".into()),
            TableCell::new("! ".into()),
            TableCell::new(format!("Fehler beim Laden der Prebuilt-Pizzen:")),
        ]),
        TableRow::new(vec![
            TableCell::new(" ".into()),
            TableCell::new(" ".into()),
            TableCell::new(format!("{err_msg}")),
        ]),
        TableRow::new(vec![
            TableCell::new(" ".into()),
            TableCell::new("=>".into()),
            TableCell::new("Bitte 'pizza_prebuilds_text' korrigieren.".into()),
        ]),
    ]);
    TableMenu::new("Order Menu (Fehler)".into(), table)
}

impl State {
    pub fn refresh_order_menu(&mut self) {
        if let Ok(catalog) = load_toppings_from_file("pizza_toppings_text") {
            self.toppings_catalog = catalog;
        }

        let idx = MenuIndex::OrderMenu.as_index();

        match load_prebuilt_pizzas_from_file("pizza_prebuilds_text", &self.toppings_catalog) {
            Ok(pizzas) => {
                self.prebuilt_pizzas = pizzas;
                self.menus[idx] = build_order_menu(&self.prebuilt_pizzas);
            }
            Err(e) => {
                self.prebuilt_pizzas.clear();
                self.menus[idx] = build_order_menu_error(&e.to_string());
            }
        }
        // if let Ok(pizzas) = load_prebuilt_pizzas_from_file("pizza_prebuilds_text", &self.toppings_catalog) {
        //     self.prebuilt_pizzas = pizzas;
        //
        //     let idx = MenuIndex::OrderMenu.as_index();
        //     self.menus[idx] = build_order_menu(&self.prebuilt_pizzas);

            let len = self.menus[idx].table_mut().rows_mut().len();
            if self.selected_row >= len { self.selected_row = 0; }
            select_row(self.menus[idx].table_mut(), self.selected_row);

    }
}
