fn main() {
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y.unwrap_or(0);
}
