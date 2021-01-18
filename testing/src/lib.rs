struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

struct Guess {
    value: i32
}

impl Guess {
    fn new(val: i32) -> Guess {
        if val < 1 || val > 100 {
            panic!("The guess must be between 1 and 100, got {}", val);
        }

        Guess { value: val }
    }
}

#[cfg(test)]
mod tests {
    use crate::Rectangle;
    use crate::Guess;

    #[test]
    fn passes() {
        assert!(true);
    }

    #[test]
    fn fails() {
        panic!("I have panicked here");
    }

    #[test]
    fn test_rectangle_can_hold() {
        let first: Rectangle = Rectangle {
            width: 10,
            height: 8
        };

        let second: Rectangle = Rectangle {
            width: 6,
            height: 12
        };

        assert!(first.can_hold(&second));
    }

    fn add_two(arg: i32) -> i32 {
        arg + 2
    }

    #[test]
    fn test_add_two() {
        assert_eq!(
            add_two(2), 5,
            "The two args were not equal"
        );
    }

    #[test]
    #[should_panic(expected = "The guess must be between 1 and 100")]
    fn test_guess_new() {
        Guess::new(200);
    }

    #[test]
    fn test_return_type() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two is not equals to four"))
        }
    }

    #[test]
    #[ignore]
    fn expensive_test() {}
}