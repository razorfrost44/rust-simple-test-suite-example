pub mod calculator;
pub mod grocery_store;
pub mod shopper;

use grocery_store::*;
use shopper::*;

pub fn run() {
    println!("START\n");

    let a = 10;
    let b = 5;
    let c = 0.0;
    let d = 3;

    let sum = calculator::add(a, b);
    let diff = calculator::subtract(a, b);
    let div = calculator::divide(a as f32, d as f32).unwrap_or(0.0);
    let div_by_zero = calculator::divide(a as f32, c).unwrap_or(0.0);
    let mul = calculator::multiply(b, d);

    println!("Calculator:\n");
    println!("Sum: {}", sum);
    println!("Difference: {}", diff);
    println!("Division: {}", div);
    println!("Division by zero: {}", div_by_zero);
    println!("Multiplication: {}", mul);

    println!("\nGrocery Store:\n");
    println!("Current Fruit: {:?}", get_current_fruit());
    println!("Current Vegetable: {:?}", get_current_vegetable());
    set_current_fruit(Fruit::Orange);
    set_current_vegetable(Vegetable::Spinach);
    println!("Current Fruit: {:?}", get_current_fruit());
    println!("Current Vegetable: {:?}", get_current_vegetable());

    println!("\nShopper\n");
    println!("Grab Fruit: {}", grab_fruit());
    println!("Grab Vegetable: {}", grab_vegetable());
    println!("Grab Both: {}", grab_fruit_and_vegetable());
    change_fruit(Fruit::Banana);
    change_vegetable(Vegetable::Broccoli);
    println!("Grab Fruit: {}", grab_fruit());
    println!("Grab Vegetable: {}", grab_vegetable());
    println!("Grab Both: {}", grab_fruit_and_vegetable());

    println!("\nEND");
}
