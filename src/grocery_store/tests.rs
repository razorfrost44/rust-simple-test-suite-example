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
