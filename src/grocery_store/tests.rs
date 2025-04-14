use super::*;

#[test]
fn test_current_fruit() {
    // Arrange
    // Act
    let result = get_current_fruit();
    // Assert
    assert_eq!(result, Fruit::Apple);
}

#[test]
fn test_set_current_fruit() {
    // Arrange
    set_current_fruit(Fruit::Banana);
    // Act
    let result = get_current_fruit();
    // Assert
    assert_eq!(result, Fruit::Banana);
}

#[test]
fn test_current_vegetable() {
    // Arrange
    // Act
    let result = get_current_vegetable();
    // Assert
    assert_eq!(result, Vegetable::Carrot);
}

#[test]
fn test_set_current_vegetable() {
    // Arrange
    set_current_vegetable(Vegetable::Spinach);
    // Act
    let result = get_current_vegetable();
    // Assert
    assert_eq!(result, Vegetable::Spinach);
}
