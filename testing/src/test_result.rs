#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Two plus three is not equal four."))
        }
    }

    #[test]
    fn it_works2() {
        assert_eq!(2 + 2, 4)
    }

    #[test]
    #[ignore]
    fn expensive_test() {}
}
