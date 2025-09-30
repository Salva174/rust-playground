use std::io::{Write, Stdout};

pub mod types;
pub mod admin;
pub mod table;
pub mod table_menu;



pub fn clear_screen(stdout: &mut Stdout) -> Result<(), Box<dyn std::error::Error>> {
    write!(stdout, "\x1B[2J\x1B[1;1H")?;
    Ok(())
}