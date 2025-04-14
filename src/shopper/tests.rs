use super::*;
use crate::grocery_store::MockGroceryStore;

struct TestContext {
    grocery_store: MockGroceryStore,
}

// Implementing any special needs for when TestContext is destroyed
impl Drop for TestContext {
    fn drop(&mut self) {
        // Clean up any resources if needed
    }
}

fn setup() -> TestContext {
    TestContext {
        grocery_store: MockGroceryStore::new(),
    }
}

#[test]
fn test_grab_fruit() {
    // Arrange
    let mut context = setup();
    context
        .grocery_store
        .expect_get_current_fruit()
        .return_const(Fruit::Apple);
    let expect = String::from("Apple");
    // Act
    let result = grab_fruit(&context.grocery_store);
    // Assert
    assert_eq!(result, expect);
}

#[test]
fn test_change_fruit() {
    // Arrange
    let mut context = setup();
    context
        .grocery_store
        .expect_set_current_fruit()
        .withf(|fruit| *fruit == Fruit::Banana)
        .return_const(());
    context
        .grocery_store
        .expect_get_current_fruit()
        .return_const(Fruit::Banana);
    // Act
    change_fruit(&context.grocery_store, Fruit::Banana);
    let result = grab_fruit(&context.grocery_store);
    let expect = String::from("Banana");
    // Assert
    assert_eq!(result, expect);
}

#[test]
fn test_grab_vegetable() {
    // Arrange
    let mut context = setup();
    context
        .grocery_store
        .expect_get_current_vegetable()
        .return_const(Vegetable::Carrot);
    // Act
    let result = grab_vegetable(&context.grocery_store);
    let expect = String::from("Carrot");
    // Assert
    assert_eq!(result, expect);
}

#[test]
fn test_change_vegetable() {
    // Arrange
    let mut context = setup();
    context
        .grocery_store
        .expect_set_current_vegetable()
        .withf(|vegetable| *vegetable == Vegetable::Broccoli)
        .return_const(());
    context
        .grocery_store
        .expect_get_current_vegetable()
        .return_const(Vegetable::Broccoli);
    // Act
    change_vegetable(&context.grocery_store, Vegetable::Broccoli);
    let result = grab_vegetable(&context.grocery_store);
    let expect = String::from("Broccoli");
    // Assert
    assert_eq!(result, expect);
}

#[test]
fn test_grab_fruit_and_vegetable() {
    // Arrange
    let mut context = setup();
    context
        .grocery_store
        .expect_get_current_fruit()
        .return_const(Fruit::Apple);
    context
        .grocery_store
        .expect_get_current_vegetable()
        .return_const(Vegetable::Carrot);
    // Act
    let result = grab_fruit_and_vegetable(&context.grocery_store);
    let expect = String::from("Apple Carrot");
    // Assert
    assert_eq!(result, expect);
}
