use std::io::{Stdout, Write};
use crate::table_menu::TableMenu;
use crate::state::{MenuIndex, State};

pub fn render_menu(
    stdout: &mut Stdout,
    menu: &TableMenu,
    menu_name: &str,
    selected_row: usize,
    footer: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(stdout, "\x1B[1J\x1B[1;1H")?;
    writeln!(stdout, "{menu}")?;
    writeln!(stdout, "Row: {selected_row}, Menu: {menu_name}")?;

    for line in footer {
        if !line.is_empty() {
            writeln!(stdout, "{line}")?;
        } else {
            writeln!(stdout)?;
        }
    }
    Ok(())
}

pub fn render(stdout: &mut Stdout, state: &State) -> Result<(), Box<dyn std::error::Error>> {
    let menu = state.current_menu();
    let (menu_name, footer): (&str, Vec<&str>) = match state.current_menu {
        MenuIndex::MainMenu => ("MainMenu", vec!["[↑/↓] bewegen · [Enter] auswählen"]),
        MenuIndex::OrderMenu => ("OrderMenu", vec!["[↑/↓] bewegen · [Enter] auswählen · [Backspace] zurück"]),
        MenuIndex::EditToppingsMenu => ("EditToppingsMenu", vec!["[↑/↓] bewegen · [Enter] auswählen · [Backspace] zurück"]),
    };
    render_menu(stdout, menu, menu_name, state.selected_row(), &footer)?;

    Ok(())
}
