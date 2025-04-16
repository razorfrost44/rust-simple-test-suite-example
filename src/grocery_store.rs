// Imports - Tests
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

// Here as a reference for how Rust handles 'destructors'
// Drop will be called when the object goes out of scope
// impl Drop for GroceryStoreImpl {
//     fn drop(&mut self) {
//         println!("Dropping GroceryStoreImpl");
//     }
// }

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

#[cfg(test)]
mod tests {
    use super::*;

    struct TestContext {
        grocery_store: GroceryStoreImpl,
    }

    // Implementing any special needs for when TestContext is destroyed
    impl Drop for TestContext {
        fn drop(&mut self) {
            // Clean up any resources if needed
        }
    }

    fn setup() -> TestContext {
        TestContext {
            grocery_store: GroceryStoreImpl::new(),
        }
    }

    #[test]
    fn test_current_fruit() {
        // Arrange
        let context = setup();
        // Act
        let result = context.grocery_store.get_current_fruit();
        // Assert
        assert_eq!(result, Fruit::Apple);
    }

    #[test]
    fn test_set_current_fruit() {
        // Arrange
        let context = setup();
        context.grocery_store.set_current_fruit(Fruit::Banana);
        // Act
        let result = context.grocery_store.get_current_fruit();
        // Assert
        assert_eq!(result, Fruit::Banana);
    }

    #[test]
    fn test_current_vegetable() {
        // Arrange
        let context = setup();
        context
            .grocery_store
            .set_current_vegetable(Vegetable::Carrot);
        // Act
        let result = context.grocery_store.get_current_vegetable();
        // Assert
        assert_eq!(result, Vegetable::Carrot);
    }

    #[test]
    fn test_set_current_vegetable() {
        // Arrange
        let context = setup();
        context
            .grocery_store
            .set_current_vegetable(Vegetable::Spinach);
        // Act
        let result = context.grocery_store.get_current_vegetable();
        // Assert
        assert_eq!(result, Vegetable::Spinach);
    }
}
