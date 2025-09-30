use std::io::{Stdout, Write};
use crate::state::State;

pub fn render(stdout: &mut Stdout, state: &State) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(stdout, "\x1B[1J\x1B[1;1H")?;
    writeln!(stdout, "{}", state.current_menu())?;
    writeln!(stdout, "Row: {}, Menu: {:?}", state.selected_row, state.current_menu)?;

    Ok(())
}


