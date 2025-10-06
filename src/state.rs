use pizzeria_lib::table::{Table, TableCell, TableRow};
use pizzeria_lib::table_menu::TableMenu;
use pizzeria_lib::types::{load_prebuild_pizzas_from_file, load_toppings_from_file, Pizza, Topping};
use crate::update::build_order_menu;

pub struct State {
    pub menus: [TableMenu; 3],
    pub current_menu: MenuIndex,
    pub selected_row: usize,
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
    let prebuilt_pizzas  = load_prebuild_pizzas_from_file("pizza_prebuilds_text", &toppings_catalog).unwrap_or_default();
    let order_menu = build_order_menu(&prebuilt_pizzas);

    // state.prebuilt_pizzas = prebuilt_pizzas;
    // state.menus[MenuIndex::OrderMenu.as_index()] = build_order_menu(&state.prebuilt_pizzas);

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
        selected_row: 0,
        toppings_catalog,
        prebuilt_pizzas,
    }
}
