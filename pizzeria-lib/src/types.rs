use std::collections::HashMap;

pub fn parse_toppings(content: &str) -> Result<Vec<Topping>, String> {
    let mut toppings = Vec::new();

    for (lineno, raw) in content.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }

        let mut split = line.split('#');
        let name = split
            .next()
            .ok_or_else(|| format!("Zeile {}: Topping-Name fehlt", lineno + 1))?;
        let price_text = split
            .next()
            .ok_or_else(|| format!("Zeile {}: Topping-Preis fehlt", lineno + 1))?;

        let price = price_text.parse::<u32>()
            .map_err(|_| format!("Zeile {}: Ungültiger Preis '{}'", lineno + 1, price_text))?;

        toppings.push(Topping {
            name: name.to_string(),
            price,
        });
    }

    Ok(toppings)
}


pub fn parse_prebuild_pizza(content: &str, available: &[Topping]) -> Result<Vec<Pizza>, String> {
    let index: HashMap<&str, &Topping> =
        available.iter().map(|t| (t.name.as_str(), t)).collect();

    let mut prebuilds = Vec::new();

    for (lineno, raw) in content.lines().enumerate() {
        let line = raw.trim();                                     
        if line.is_empty() {
            continue;
        }

        // Format: <Pizza-Name>#<Topping1|Topping2|…>#<Basispreis>                                      //Topping muss in "pizza_toppings_text" enthalten sein!
        let mut split = line.splitn(3,'#');
        let name = split
            .next()
            .ok_or_else(|| format!("Zeile {}: Pizza-Name fehlt", lineno + 1))?;
        let topping_names = split
            .next()
            .unwrap_or("-");
        let base_price_text = split
            .next()
            .ok_or_else(|| format!("Zeile {}: Basispreis fehlt", lineno + 1))?;
        let base_price = base_price_text.parse::<u32>()
            .map_err(|_| format!("Zeile {}: Ungültiger Basispreis '{}'", lineno + 1, base_price_text))?;

        let toppings: Vec<Topping> = if topping_names.trim().is_empty() || topping_names == "-" {
            Vec::new()
        } else {
            let mut topping_list = Vec::new();
            for name_of_topping in topping_names
                .split('|')
                .map(|raw_name| raw_name.trim())
                .filter(|raw_name| !raw_name.is_empty())            //Wenn Topping-Feld leer, wird es nicht aufgenommen
            {
                match index.get(name_of_topping) {
                    Some(topping) => topping_list.push((*topping).clone()),
                    None => return Err(format!("Zeile {}: Unbekanntes Topping '{}'", lineno + 1, name_of_topping)),
                }
            }
            topping_list
        };

        prebuilds.push(Pizza {
            name: name.to_string(),
            toppings,
            base_price,
        });
    }

    Ok(prebuilds)
}

#[derive(Debug, PartialEq)]
pub struct Topping {
    pub name: String,
    pub price: u32
}

impl Topping {

    pub fn shortname(&self) -> char {
        self.name.chars().next().expect("There must be a topping-name!")
    }
}

impl Clone for Topping {
    fn clone(&self) -> Self {
        Topping {
            name: Clone::clone(&self.name),
            price: self.price
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Pizza {
    pub name: String,
    pub toppings: Vec<Topping>,
    pub base_price: u32,
}

impl Pizza {

    pub fn total_price(&self) -> u32 {
        let total_topping_price = self.toppings.iter()
            .map(|topping| topping.price)
            .sum::<u32>();
        self.base_price + total_topping_price
    }
}

#[cfg(test)]
mod tests {
    use crate::types::*;

    #[test]
    fn test_total_price_of_a_pizza_without_toppings() {

        let pizza = Pizza {
            name: String::new(),
            toppings: vec![],
            base_price: 8,
        };

        assert_eq!(pizza.total_price(), 8);
    }

    #[test]
    fn test_total_price_of_a_pizza_with_toppings() {

        let pizza = Pizza {
            name: String::new(),
            toppings: vec![
                Topping { name: String::from("Test_Cheese"), price: 14 },
                Topping { name: String::from("Test_Tomato"), price: 30}
            ],
            base_price: 10
        };

        assert_eq!(pizza.total_price(), 54);
    }

    #[test]
    fn test_parse_toppings_from_empty_string() -> Result<(), Box<dyn std::error::Error>> {

        let toppings = parse_toppings("")?;

        assert_eq!(toppings, Vec::new());

        Ok(())
    }

    #[test]
    fn test_parse_toppings_from_string_with_single_topping() -> Result<(), Box<dyn std::error::Error>> {

        let toppings = parse_toppings(r#"
Ham#8
        "#)?;

        assert_eq!(toppings, vec![
            Topping { name: String::from("Ham"), price: 8 }
        ]);

        Ok(())
    }

    #[test]
    fn test_parse_toppings_from_string_with_multiple_toppings() -> Result<(), Box<dyn std::error::Error>> {

        let toppings = parse_toppings(r#"
Ham#8
Cheese#3
Brocoli#4
        "#)?;

        assert_eq!(toppings, vec![
            Topping { name: String::from("Ham"), price: 8 },
            Topping { name: String::from("Cheese"), price: 3 },
            Topping { name: String::from("Brocoli"), price: 4 },
        ]);

        Ok(())
    }
}
