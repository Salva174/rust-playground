use std::{fs, io};
use pizzeria_lib::table::{Table, TableCell, TableRow};
use pizzeria_lib::table::Align::Right;
use pizzeria_lib::table_menu::TableMenu;
use pizzeria_lib::types::{load_toppings_from_file, parse_prebuild_pizza, Pizza, Topping};

pub struct State {
    pub menus: [TableMenu; 3],
    pub current_menu: MenuIndex,
    pub selected_rows: [usize; 3],
    pub toppings_catalog: Vec<Topping>,
    pub prebuilt_pizzas: Vec<Pizza>,
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

pub fn create_initial_state() -> State {
    let toppings_catalog = load_toppings_from_file("pizza_toppings_text").unwrap_or_default();

    let (prebuilt_pizzas, order_menu) = match load_prebuilt_pizzas_from_file("pizza_prebuilds_text", &toppings_catalog) {
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
    }
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
