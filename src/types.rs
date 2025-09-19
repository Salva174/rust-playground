


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
}
