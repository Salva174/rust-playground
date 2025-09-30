use pizzeria_lib::table::Table;
use crate::input::InputEvent;
use crate::state::{MenuIndex, State};

pub fn update(input: InputEvent, state: &mut State) -> bool {

    if let InputEvent::Exit = input {
        return true;
    }

    match state.current_menu {
        MenuIndex::MainMenu => main_menu_update(input, state),
        MenuIndex::EditToppingsMenu => edit_toppings_menu_update(input, state),
        MenuIndex::OrderMenu => order_menu_update(input, state),
    }
}

fn order_menu_update(input: InputEvent, state: &mut State) -> bool {

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
                0 => state.current_menu = MenuIndex::OrderMenu,
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

fn edit_toppings_menu_update(input: InputEvent, state: &mut State) -> bool {

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
            todo!()
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