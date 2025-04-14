// Imports - Tests
#[cfg(test)]
mod tests;
use mockall::automock;
// Imports - Rust
use std::sync::Mutex;
// Imports - creates
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Fruit {
    Apple,
    Banana,
    Orange,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Vegetable {
    Carrot,
    Broccoli,
    Spinach,
}

#[automock]
pub trait GroceryStore {
    fn get_current_fruit(&self) -> Fruit;
    fn set_current_fruit(&self, fruit: Fruit);
    fn get_current_vegetable(&self) -> Vegetable;
    fn set_current_vegetable(&self, vegetable: Vegetable);
}

pub struct GroceryStoreImpl {
    current_fruit: Mutex<Fruit>,
    current_vegetable: Mutex<Vegetable>,
}

impl GroceryStoreImpl {
    pub fn new() -> Self {
        GroceryStoreImpl {
            current_fruit: Mutex::new(Fruit::Apple),
            current_vegetable: Mutex::new(Vegetable::Carrot),
        }
    }
}

impl GroceryStore for GroceryStoreImpl {
    fn get_current_fruit(&self) -> Fruit {
        self.current_fruit.lock().unwrap().clone()
    }

    fn set_current_fruit(&self, fruit: Fruit) {
        let mut fruit_global = self.current_fruit.lock().unwrap();
        *fruit_global = fruit.clone();
    }

    fn get_current_vegetable(&self) -> Vegetable {
        self.current_vegetable.lock().unwrap().clone()
    }

    fn set_current_vegetable(&self, vegetable: Vegetable) {
        let mut vegetable_global = self.current_vegetable.lock().unwrap();
        *vegetable_global = vegetable.clone();
    }
}
