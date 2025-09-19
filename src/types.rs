
pub fn parse_toppings(content: String) -> Vec<Topping> {

    let mut toppings  = Vec::<Topping>::new();

    for line in content.split('\n') {
        if line.trim().is_empty() {
            continue
        }
        let mut split = line.split(' ');
        let name = split.next().expect("There must be a topping name!");
        let price_text = split.next().expect("There must be a topping price!");
        let price = price_text.parse::<u32>().expect("Price must be a number!");
        toppings.push(Topping { name: String::from(name), price });
    }

    toppings
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
    fn test_parse_toppings_from_empty_string() {

        let toppings = parse_toppings(String::new());

        assert_eq!(toppings, Vec::new())
    }

    #[test]
    fn test_parse_toppings_from_string_with_single_topping() {

        let toppings = parse_toppings(String::from(r#"
Ham 8
        "#));

        assert_eq!(toppings, vec![
            Topping { name: String::from("Ham"), price: 8 }
        ])
    }

    #[test]
    fn test_parse_toppings_from_string_with_multiple_toppings() {

        let toppings = parse_toppings(String::from(r#"
Ham 8
Cheese 3
Brocoli 4
        "#));

        assert_eq!(toppings, vec![
            Topping { name: String::from("Ham"), price: 8 },
            Topping { name: String::from("Cheese"), price: 3 },
            Topping { name: String::from("Brocoli"), price: 4 },
        ])
    }
}
