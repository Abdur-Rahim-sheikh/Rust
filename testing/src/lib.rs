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
}
