use std::fmt::{ Display, Formatter};

pub struct Menu {
    title: String,
    entries: Vec<String>,
}

impl Menu {
    pub fn new(title: String, entries: Vec<String>) -> Self {
        Self { title, entries }
    }
}

impl Display for Menu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut max_length = self.title.len();
        for entry in self.entries.iter() {
            if entry.len() > max_length {
                max_length = entry.len();
            }
        }
        let min_width = max_length + 2;
        let top = format!("┌{}┐", "─".repeat(min_width));
        let bottom = format!("└{}┘", "─".repeat(min_width));

        // let text_side = format!("│{}│\n", self.text[0..self.width].to_string());
        let mut x = Vec::<String>::new();
        for entry in self.entries.iter() {
            let padding_size = max_length - entry.len();
            let padding = " ".repeat(padding_size);
            x.push(format!("│ {entry}{padding} │"));
        }
        let y = x.join("\n");

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
        if !y.is_empty() {
            writeln!(f, "{y}")?;
        }
        writeln!(f, "{bottom}")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::console::Menu;

    #[test]
    fn test_empty_menu() -> Result<(), Box<dyn std::error::Error>> {

        let title_text = String::from("Menu");
        let panel = Menu::new(title_text, vec![]);

        println!("{panel}");

        assert_eq!(format!("{panel}").trim(), "
┌──────┐
│ Menu │
├──────┤
└──────┘
        ".trim());

        Ok(())
    }

    #[test]
    fn test_menu_with_entries() -> Result<(), Box<dyn std::error::Error>> {

        let title_text = String::from("Menu");
        let panel = Menu::new(title_text, vec![
            String::from("1. Pizza Margherita"),
            String::from("2. Pizza Pepperoni"),
            String::from("3. Pizza Salami")
        ]);

        println!("{panel}");

        assert_eq!(format!("{panel}").trim(), "
┌─────────────────────┐
│        Menu         │
├─────────────────────┤
│ 1. Pizza Margherita │
│ 2. Pizza Pepperoni  │
│ 3. Pizza Salami     │
└─────────────────────┘
                ".trim());

        Ok(())
    }

    #[test]
    fn test_menu_with_long_title_and_short_entries() -> Result<(), Box<dyn std::error::Error>> {

        let panel = Menu::new(String::from("Welcome to Salvatores Pizza!"), vec![
            String::from("1. M"),
        ]);

        println!("{panel}");

        Ok(())
    }
}