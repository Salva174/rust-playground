use pizzeria_lib::table::{Table, TableCell, TableRow};
use pizzeria_lib::table_menu::TableMenu;

pub struct State {
    pub menus: [TableMenu; 3],
    pub current_menu: MenuIndex,
    pub selected_row: usize,
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
            TableMenu::new(String::from("Order Menu"), Table::new(vec! [
                TableRow::new( vec! [
                    TableCell::new(String::from(">")),
                    TableCell::new(String::from("1:")),
                    TableCell::new(String::from("Pizza Salami"))
                ]),
                TableRow::new( vec! [
                    TableCell::new(String::from(" ")),
                    TableCell::new(String::from("2:")),
                    TableCell::new(String::from("Pizza Hawaii"))
                ]),
                TableRow::new( vec! [
                    TableCell::new(String::from(" ")),
                    TableCell::new(String::from("3:")),
                    TableCell::new(String::from("Pizza funghi"))
                ])
            ])),
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
                    TableCell::new(String::from("D:")),
                    TableCell::new(String::from("Delete-List"))
                ])
            ])),
        ],
        current_menu: MenuIndex::MainMenu,
        selected_row: 0
    }
}
