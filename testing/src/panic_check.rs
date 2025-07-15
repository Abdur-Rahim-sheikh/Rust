pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("guess greater than 1")
        } else if value > 100 {
            panic!("guess less than 100")
        }
        return Guess { value };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "guess less than 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
