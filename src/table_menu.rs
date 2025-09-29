use std::cmp::max;
use std::fmt::{Display, Formatter};
use crate::table::{render_table, Table};

pub struct TableMenu {
    title: String,
    table: Table,
}

impl TableMenu {
    pub fn new(title: String, table: Table) -> Self {
        Self { title, table }
    }
}

impl Display for TableMenu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut table_buffer = String::new();
        render_table(&self.table, &mut table_buffer);
        let mut max_length = max(
            self.title.len(),
            table_buffer.split('\n').next().unwrap_or_default().len()
        );

        let min_width = max_length + 4;
        let top = format!("┌{}┐", "─".repeat(min_width));
        let bottom = format!("└{}┘", "─".repeat(min_width));

        let title_text = {
            let padding = min_width - self.title.len();
            let left_padding =  padding / 2;
            let right_padding = (padding as f32 / 2_f32).ceil() as usize;
            format!("│{}{}{}│\n", " ".repeat(left_padding), self.title, " ".repeat(right_padding))
        };
        let divider = format!("├{}┤\n", "─".repeat(min_width ));

        writeln!(f,"{top}")?;
        write!(f, "{title_text}")?;
        write!(f, "{divider}")?;
        for table_line in table_buffer.split("\n") {
            if !table_line.is_empty() {
                let padding_size = max_length - table_line.len();
                let padding = "+".repeat(padding_size);
                writeln!(f, "│  {table_line}{padding}  │")?;
            }
        }
        writeln!(f, "{bottom}")?;


        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::table::{Align, Table, TableCell, TableRow};
    use crate::table_menu::TableMenu;

    #[test]
    fn test() -> Result<(), Box<dyn std::error::Error>> {
        let title_text = String::from("Menu-Title");
        let table = Table::new(vec! [
            TableRow::new( vec! [
                TableCell::new(String::from("1.")),
                TableCell::new(String::from("Salami")),
                TableCell::new_with_alignment(String::from("3.00$"), Align::Right)
            ]),
            TableRow::new(vec! [
                TableCell::new(String::from("2.")),
                TableCell::new(String::from("Broccoli")),
                TableCell::new_with_alignment(String::from("400.00$"), Align::Left)
            ])
        ]);
        let table_menu = TableMenu::new(title_text, table);
        println!("{table_menu}");
        Ok(())
    }
}