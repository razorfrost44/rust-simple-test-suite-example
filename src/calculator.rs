pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn divide(a: f32, b: f32) -> Result<f32, String> {
    if b == 0.0 {
        return Err(format!("Division by zero error: {} / {}", a, b));
    }

    Ok(a / b)
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
