use super::*;

trait GroceryStore {
    fn get_current_fruit(&self) -> Fruit;
    fn set_current_fruit(&self, fruit: Fruit);
    fn get_current_vegetable(&self) -> Vegetable;
    fn set_current_vegetable(&self, vegetable: Vegetable);
}

#[test]
fn test_grab_fruit() {}

#[test]
fn test_change_fruit() {}

#[test]
fn test_grab_vegetable() {}

#[test]
fn test_change_vegetable() {}

#[test]
fn test_grab_fruit_and_vegetable() {}
