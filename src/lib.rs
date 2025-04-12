pub mod calculator;

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

    println!("Sum: {}", sum);
    println!("Difference: {}", diff);
    println!("Division: {}", div);
    println!("Division by zero: {}", div_by_zero);
    println!("Multiplication: {}", mul);

    println!("\nEND");
}
