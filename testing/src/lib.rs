pub mod panic_check;

#[cfg(test)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
#[cfg(test)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

pub fn add_two(a: i32) -> i32 {
    return a + 2;
}

pub fn greeting(name: &str) -> String {
    return format!("Hello {name}!");
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larget_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
        assert_ne!(5, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let name = "Carol";
        let result = greeting(name);
        assert!(
            result.contains(name),
            "Greeting did not contain name. result={result}"
        );
        assert!(
            !result.contains("testi"),
            "Greeting did contain name. result={result}"
        );
    }
}
