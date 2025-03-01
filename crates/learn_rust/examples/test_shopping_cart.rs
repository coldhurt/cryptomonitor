use std::ops::Add;

struct Item {
    name: String,
    price: f64,
    item_type: ItemType,
}

#[derive(Debug)]
enum ItemType {
    Vegetable,
    Fruit,
    Meat,
}

struct ShoppingCart {
    items: Vec<Item>,
}

impl Add<Item> for ShoppingCart {
    type Output = Self;

    fn add(mut self, item: Item) -> Self {
        self.add_item(item);
        self
    }
}

impl ShoppingCart {
    fn new() -> Self {
        ShoppingCart { items: vec![] }
    }

    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    fn total_price(&self) -> f64 {
        self.items.iter().map(|item| item.price).sum()
    }

    fn print_items(&self) {
        for item in self.items.iter() {
            println!(
                "Item: {}, Type: {:?}, Price: {}",
                item.name, item.item_type, item.price
            );
        }
    }
}

fn main() {
    let mut shopping_cart = ShoppingCart::new();
    let apple = Item {
        name: "Apple".to_string(),
        price: 1.0,
        item_type: ItemType::Fruit,
    };
    let banana = Item {
        name: "Banana".to_string(),
        price: 2.0,
        item_type: ItemType::Fruit,
    };
    let cumumber = Item {
        name: "Cumumber".to_string(),
        price: 1.5,
        item_type: ItemType::Vegetable,
    };
    let beef = Item {
        name: "Beef".to_string(),
        price: 3.0,
        item_type: ItemType::Meat,
    };

    // shopping_cart.add_item(apple);
    // shopping_cart.add_item(banana);
    // shopping_cart.add_item(cumumber);
    // shopping_cart.add_item(beef);

    shopping_cart = shopping_cart + apple + banana + cumumber + beef;
    shopping_cart.print_items();
    println!("Total price: {}", shopping_cart.total_price());
}
