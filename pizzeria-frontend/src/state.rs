use std::io::Write;
use std::{fs, io};
use std::io::Stdout;
use crate::Arguments;
use crate::table::{Table, TableCell, TableRow};
use crate::table::Align::Right;
use crate::table_menu::TableMenu;
use crate::types::{parse_prebuild_pizza, parse_toppings, Pizza, Topping};
use crate::http::{read_pizza_prebuilds, read_toppings};

pub struct State {
    pub menus: [TableMenu; 3],
    pub current_menu: MenuIndex,
    pub selected_rows: [usize; 3],
    pub toppings_catalog: Vec<Topping>,
    pub prebuilt_pizzas: Vec<Pizza>,
    pub pending_fallbacks: Vec<String>,
}

impl State {
    pub fn current_menu(&self) -> &TableMenu {
        &self.menus[self.current_menu.as_index()]
    }

    pub fn current_menu_mut(&mut self) -> &mut TableMenu {
        &mut self.menus[self.current_menu.as_index()]
    }

    pub fn selected_row(&self) -> usize {
        self.selected_rows[self.current_menu.as_index()]
    }
    pub fn set_selected_row(&mut self, row: usize) {
        let i = self.current_menu.as_index();
        self.selected_rows[i] = row;
    }

    pub fn apply_selection_marker(&mut self) {
        let sel = self.selected_row();
        let table = self.current_menu_mut().table_mut();
        crate::update::select_row(table, sel);
    }

    pub fn refresh_order_menu(&mut self, arguments: &Arguments) {
        if let Ok(catalog) = load_toppings_from_backend(arguments) {
            self.toppings_catalog = catalog;
        }

        let idx = MenuIndex::OrderMenu.as_index();

        match load_prebuilt_pizzas_from_backend(&self.toppings_catalog, arguments) {
            Ok(pizzas) => {
                self.prebuilt_pizzas = pizzas;
                self.menus[idx] = build_order_menu(&self.prebuilt_pizzas);
            }
            Err(e) => {
                self.prebuilt_pizzas.clear();
                self.menus[idx] = build_order_menu_error(&e.to_string());
            }
        }

        let len = self.menus[idx].table_mut().rows_mut().len();
        if self.selected_rows[idx] >= len {
            self.selected_rows[idx] = 0;
        }
        if matches!(self.current_menu, MenuIndex::OrderMenu) {
            let sel_row = self.selected_rows[idx];
            crate::update::select_row(self.menus[idx].table_mut(), sel_row);
        }
    }
}

#[derive(Debug)]
pub enum MenuIndex {
    MainMenu,
    OrderMenu,
    EditToppingsMenu,
}

impl MenuIndex {
    pub fn as_index(&self) -> usize {
        match self {
            MenuIndex::MainMenu => 0,
            MenuIndex::OrderMenu => 1,
            MenuIndex::EditToppingsMenu => 2,
        }
    }
}

pub fn create_initial_state(arguments: &Arguments) -> State {
    let toppings_catalog = load_toppings_from_backend(arguments).unwrap_or_default();

    let (prebuilt_pizzas, order_menu) = match load_prebuilt_pizzas_from_backend(&toppings_catalog, arguments) {
        Ok(pz) => {
            let menu = build_order_menu(&pz);
            (pz, menu)
        }
        Err(e) => {
            let menu = build_order_menu_error(&e.to_string());
            (Vec::new(), menu)
        }
    };

    State {
        menus: [
            TableMenu::new(String::from("Welcome to Salvatores Pizza!"), Table::new(vec! [
                TableRow::new( vec! [
                    TableCell::new(String::from(">")),
                    TableCell::new(String::from("1:")),
                    TableCell::new(String::from("Order Pizza"))
                ]),
                TableRow::new( vec! [
                    TableCell::new(String::from(" ")),
                    TableCell::new(String::from("2:")),
                    TableCell::new(String::from("Edit Toppings"))
                ]),
                TableRow::new( vec! [
                    TableCell::new(String::from(" ")),
                    TableCell::new(String::from("3:")),
                    TableCell::new(String::from("Quit"))
                ])
            ])),
            order_menu,
            TableMenu::new(String::from("Edit Toppings Menu"), Table::new(vec! [
                TableRow::new( vec! [
                    TableCell::new(String::from(">")),
                    TableCell::new(String::from("A:")),
                    TableCell::new(String::from("Add Pizza-Topping"))
                ]),
                TableRow::new( vec! [
                    TableCell::new(String::from(" ")),
                    TableCell::new(String::from("R:")),
                    TableCell::new(String::from("Remove Pizza-Topping"))
                ]),
                TableRow::new( vec! [
                    TableCell::new(String::from(" ")),
                    TableCell::new(String::from("T:")),
                    TableCell::new(String::from("Show Topping-List"))
                ]),
                TableRow::new( vec! [
                    TableCell::new(String::from(" ")),
                    TableCell::new(String::from("D:")),
                    TableCell::new(String::from("Delete-List"))
                ])
            ])),
        ],
        current_menu: MenuIndex::MainMenu,
        selected_rows: [0, 0, 0],
        toppings_catalog,
        prebuilt_pizzas,
        pending_fallbacks: Vec::new(),
    }
}

pub fn load_toppings_from_backend(arguments: &Arguments) -> io::Result<Vec<Topping>> {
    let body = read_toppings(arguments)?;
    parse_toppings(&body)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

pub fn load_prebuilt_pizzas_from_backend(available: &[Topping], arguments: &Arguments) -> io::Result<Vec<Pizza>> {
    let body = read_pizza_prebuilds(arguments)?;
    parse_prebuild_pizza(&body, available)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

pub fn load_toppings_from_file(path: &str) -> io::Result<Vec<Topping>> {
    let content = fs::read_to_string(path)?;
    parse_toppings(&content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

pub fn load_prebuilt_pizzas_from_file(path: &str, available:  &[Topping]) -> io::Result<Vec<Pizza>> {
    let content = fs::read_to_string(path)?;
    parse_prebuild_pizza(&content, available)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
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
                TableCell::new(p.name.to_string()),
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
            TableCell::new(String::from("Fehler beim Laden der Prebuilt-Pizzen:")),
        ]),
        TableRow::new(vec![
            TableCell::new(" ".into()),
            TableCell::new(" ".into()),
            TableCell::new(String::from(err_msg)),
        ]),
        TableRow::new(vec![
            TableCell::new(" ".into()),
            TableCell::new("=>".into()),
            TableCell::new("Bitte 'pizza_prebuilds_text' korrigieren.".into()),
        ]),
    ]);
    TableMenu::new("Order Menu (Fehler)".into(), table)
}

#[derive(Clone)]
pub struct TransactionRecord {
    pub price_cents: u32,
    pub name: String,
}

pub fn process_transaction_fallbacks(state: &mut State, stdout: &mut Stdout) {
    let mut still_pending = Vec::new();
    const LOG_PATH: &str = "transactions.log";

    for transaction_record in state.pending_fallbacks.drain(..) {
        if let Err(e) = append_line_sync(LOG_PATH, &transaction_record) {
            writeln!(stdout, "Warnung: Fallback-Loggen fehlgeschlagen: {e}").ok();
            still_pending.push(transaction_record);
        }
    }

    state.pending_fallbacks = still_pending;
}

//sicherstellen, dass Zeile mit \n endet
pub fn append_line_sync(path: &str, line: &str) -> std::io::Result<()> {
    use std::io::Write;

    let mut buf = line.as_bytes().to_vec();

    if !buf.ends_with(b"\n") { buf.push(b'\n'); }
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    file.write_all(&buf)
}
