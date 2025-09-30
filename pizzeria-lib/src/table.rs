pub struct Table {
    rows: Vec<TableRow>,
}

impl Table {
    pub fn new(rows: Vec<TableRow>) -> Self {
        Self { rows }
    }

    pub fn rows_mut(&mut self) -> &mut[TableRow] {
        self.rows.as_mut_slice()
    }

    pub fn push(&mut self, row: TableRow) {
        self.rows.push(row)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

pub struct TableRow {
    cells: Vec<TableCell>,
}

impl TableRow {
    pub fn new(cells: Vec<TableCell>) -> Self {
        Self { cells }
    }

    pub fn cells_mut(&mut self) -> &mut[TableCell] {
        self.cells.as_mut_slice()
    }
}

pub struct TableCell {
    text: String,
    align: Align
}

impl TableCell {
    pub fn new(text: String) -> Self {
        Self { text, align: Align::Left }
    }
    pub fn new_with_alignment(text: String, align: Align) -> Self {
        Self { text, align }
    }

    pub fn text_mut(&mut self) -> &mut String {
        &mut self.text
    }
}

pub enum Align {
    Left,
    Right
}

pub fn render_table(table: &Table, buffer: &mut String) {

    let mut columns_width = Vec::<usize>::new();
    for row in table.rows.iter() {
        for (index, column) in row.cells.iter().enumerate() {
            let text_length = column.text.len();
            if let Some(entry) = columns_width.get_mut(index) {
                if *entry < text_length {
                    *entry = text_length;
                }
            } else {
                columns_width.insert(index, text_length)
            }
        }
    }

    for row in table.rows.iter() {
        for (index, column) in row.cells.iter().enumerate() {
            let column_width = columns_width[index];
            let padding = if index < row.cells.len() - 1 {
                column_width - column.text.len() + 2
            } else {
                column_width - column.text.len()
            };
            match column.align {
                Align::Left => {
                    buffer.push_str(column.text.as_str());
                    buffer.push_str(" ".repeat(padding).as_str());
                }
                Align::Right => {
                    buffer.push_str(" ".repeat(padding).as_str());
                    buffer.push_str(column.text.as_str());
                }
            }

        }
        buffer.push('\n');
    }
}

#[cfg(test)]
mod tests {
    use crate::table::*;

    #[test]
    fn test() -> Result<(), Box<dyn std::error::Error>> {

        let table = Table {
            rows: vec![
                TableRow {
                    cells: vec![
                        TableCell { text: String::from("1."), align: Align::Left },
                        TableCell { text: String::from("Mushrooms"), align: Align::Left },
                        TableCell { text: String::from("3.00$"), align: Align::Right },
                    ]
                },
                TableRow {
                    cells: vec![
                        TableCell { text: String::from("2."), align: Align::Left },
                        TableCell { text: String::from("Onions"), align: Align::Left },
                        TableCell { text: String::from("22.00$"), align: Align::Right },
                    ]
                },
            ]
        };

        let mut buffer = String::new();

        render_table(&table, &mut buffer);

        println!("{buffer}");

        Ok(())
    }

}

