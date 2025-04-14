// Imports - Tests
#[cfg(test)]
mod tests;
// Imports - modules
use crate::grocery_store;
use grocery_store::{Fruit, GroceryStore, Vegetable};
use serde_json::{from_str, to_string};

pub fn grab_fruit(store: &dyn GroceryStore) -> String {
    let fruit = store.get_current_fruit();

    let json_str = to_string(&fruit).unwrap();
    from_str(&json_str).unwrap()
}

pub fn change_fruit(store: &dyn GroceryStore, fruit: Fruit) {
    store.set_current_fruit(fruit);
}

pub fn grab_vegetable(store: &dyn GroceryStore) -> String {
    let vegetable = store.get_current_vegetable();

    let json_str = to_string(&vegetable).unwrap();
    from_str(&json_str).unwrap()
}

pub fn change_vegetable(store: &dyn GroceryStore, vegetable: Vegetable) {
    store.set_current_vegetable(vegetable);
}

pub fn grab_fruit_and_vegetable(store: &dyn GroceryStore) -> String {
    let fruit = grab_fruit(store);
    let vegetable = grab_vegetable(store);

    format!("{} {}", fruit, vegetable)
}
