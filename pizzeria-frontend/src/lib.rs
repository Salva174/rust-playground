use std::io::Write;
use std::io::Stdout;

pub mod input;
pub mod render;
pub mod state;
pub mod update;
pub mod custom_toppings;
pub mod http;
mod ui;
mod transactions;
pub mod toppings;
pub mod table;
pub mod table_menu;
pub mod types;

pub fn clear_screen(stdout: &mut Stdout) -> Result<(), Box<dyn std::error::Error>> {
    write!(stdout, "\x1B[2J\x1B[1;1H")?;
    Ok(())
}