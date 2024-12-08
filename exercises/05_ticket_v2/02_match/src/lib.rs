enum Shape {
    Circle,
    Square,
    Rectangle,
    Triangle,
    Pentagon,
}

impl Shape {
    // TODO: Implement the `n_sides` method using a `match`.
    pub fn n_sides(&self) -> u8 {
        match self {
            Shape::Circle => 0,
            Shape::Square => 4,
            Shape::Rectangle => 4,
            Shape::Triangle => 3,
            Shape::Pentagon => 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        assert_eq!(Shape::Circle.n_sides(), 0);
    }

    #[test]
    fn test_square() {
        assert_eq!(Shape::Square.n_sides(), 4);
    }

    #[test]
    fn test_rectangle() {
        assert_eq!(Shape::Rectangle.n_sides(), 4);
    }

    #[test]
    fn test_triangle() {
        assert_eq!(Shape::Triangle.n_sides(), 3);
    }

    #[test]
    fn test_pentagon() {
        assert_eq!(Shape::Pentagon.n_sides(), 5);
    }
}
