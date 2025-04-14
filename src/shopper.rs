// Imports - Tests
#[cfg(test)]
mod tests;
// Imports - modules
use crate::grocery_store;
use grocery_store::{
    Fruit, Vegetable, get_current_fruit, get_current_vegetable, set_current_fruit,
    set_current_vegetable,
};
// Imports - creates
use serde_json::to_string;

pub fn grab_fruit() -> String {
    let fruit = get_current_fruit();

    to_string(&fruit).unwrap()
}

pub fn change_fruit(fruit: Fruit) {
    set_current_fruit(fruit);
}

pub fn grab_vegetable() -> String {
    let vegetable = get_current_vegetable();

    serde_json::to_string(&vegetable).unwrap()
}

pub fn change_vegetable(vegetable: Vegetable) {
    set_current_vegetable(vegetable);
}

pub fn grab_fruit_and_vegetable() -> String {
    let fruit = grab_fruit();
    let vegetable = grab_vegetable();

    format!("{} {}", fruit, vegetable)
}
