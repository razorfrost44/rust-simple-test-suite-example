// Imports - Rust
use std::sync::Mutex;
// Imports - creates
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Fruit {
    Apple,
    Banana,
    Orange,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Vegetable {
    Carrot,
    Broccoli,
    Spinach,
}

lazy_static! {
    static ref CURRENT_FRUIT: Mutex<Fruit> = Mutex::new(Fruit::Apple);
    static ref CURRENT_VEGETABLE: Mutex<Vegetable> = Mutex::new(Vegetable::Carrot);
}

pub fn get_current_fruit() -> Fruit {
    CURRENT_FRUIT.lock().unwrap().clone()
}

pub fn set_current_fruit(fruit: Fruit) {
    let mut fruit_global = CURRENT_FRUIT.lock().unwrap();
    *fruit_global = fruit.clone();
}

pub fn get_current_vegetable() -> Vegetable {
    CURRENT_VEGETABLE.lock().unwrap().clone()
}

pub fn set_current_vegetable(vegetable: Vegetable) {
    let mut vegetable_global = CURRENT_VEGETABLE.lock().unwrap();
    *vegetable_global = vegetable.clone();
}
