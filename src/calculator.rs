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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        // Arrange
        let a = 10;
        let b = 5;
        let c = 3;
        // Act
        let result1 = add(a, a);
        let result2 = add(a, b);
        let result3 = add(a, c);
        let result4 = add(b, b);
        let result5 = add(b, a);
        let result6 = add(b, c);
        let result7 = add(c, c);
        let result8 = add(c, a);
        let result9 = add(c, b);
        // Assert
        assert_eq!(result1, 20);
        assert_eq!(result2, 15);
        assert_eq!(result3, 13);
        assert_eq!(result4, 10);
        assert_eq!(result5, 15);
        assert_eq!(result6, 8);
        assert_eq!(result7, 6);
        assert_eq!(result8, 13);
        assert_eq!(result9, 8);
    }

    #[test]
    fn test_subtract() {
        // Arrange
        let a = 10;
        let b = 5;
        let c = 3;
        // Act
        let result1 = subtract(a, a);
        let result2 = subtract(a, b);
        let result3 = subtract(a, c);
        let result4 = subtract(b, b);
        let result5 = subtract(b, a);
        let result6 = subtract(b, c);
        let result7 = subtract(c, c);
        let result8 = subtract(c, a);
        let result9 = subtract(c, b);
        // Assert
        assert_eq!(result1, 0);
        assert_eq!(result2, 5);
        assert_eq!(result3, 7);
        assert_eq!(result4, 0);
        assert_eq!(result5, -5);
        assert_eq!(result6, 2);
        assert_eq!(result7, 0);
        assert_eq!(result8, -7);
        assert_eq!(result9, -2);
    }

    #[test]
    fn test_divide() {
        // Arrange
        let a = 10;
        let b = 5.0;
        let c = 3.0;
        let d = 0.0;
        // Act
        let result1 = divide(a as f32, b).unwrap_or(0.0);
        let result2 = divide(a as f32, c).unwrap_or(0.0);
        let result3 = divide(a as f32, d).unwrap_or(0.0);
        let result4 = divide(b, a as f32).unwrap_or(0.0);
        let result5 = divide(b, c).unwrap_or(0.0);
        let result6 = divide(b, d).unwrap_or(0.0);
        let result7 = divide(c, a as f32).unwrap_or(0.0);
        let result8 = divide(c, b).unwrap_or(0.0);
        let result9 = divide(c, d).unwrap_or(0.0);
        let result10 = divide(d, a as f32).unwrap_or(0.0);
        let result11 = divide(d, b).unwrap_or(0.0);
        let result12 = divide(d, c).unwrap_or(0.0);
        // Assert
        assert_eq!(result1, 2.0);
        assert_eq!(result2, 3.3333333);
        assert_eq!(result3, 0.0);
        assert_eq!(result4, 0.5);
        assert_eq!(result5, 1.6666666);
        assert_eq!(result6, 0.0);
        assert_eq!(result7, 0.3);
        assert_eq!(result8, 0.6);
        assert_eq!(result9, 0.0);
        assert_eq!(result10, 0.0);
        assert_eq!(result11, 0.0);
        assert_eq!(result12, 0.0);
    }

    #[test]
    fn test_multiply() {
        // Arrange
        let a = 10;
        let b = 5;
        let c = 3;
        // Act
        let result1 = multiply(a, a);
        let result2 = multiply(a, b);
        let result3 = multiply(a, c);
        let result4 = multiply(b, b);
        let result5 = multiply(b, a);
        let result6 = multiply(b, c);
        let result7 = multiply(c, c);
        let result8 = multiply(c, a);
        let result9 = multiply(c, b);
        // Assert
        assert_eq!(result1, 100);
        assert_eq!(result2, 50);
        assert_eq!(result3, 30);
        assert_eq!(result4, 25);
        assert_eq!(result5, 50);
        assert_eq!(result6, 15);
        assert_eq!(result7, 9);
        assert_eq!(result8, 30);
        assert_eq!(result9, 15);
    }
}
