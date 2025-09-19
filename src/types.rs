


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
    pub toppings: Vec<Topping>
}