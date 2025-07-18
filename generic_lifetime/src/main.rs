fn main() {
    let s1 = String::from("abcd");
    let result;
    {
        let s2 = String::from("xyz");

        result = longest(s1.as_str(), s2.as_str());
        println!("The longest string is {result}")
    }
    // println!("The longest string is {result}")
}
// &i32             // a reference
// &'a i32          // a reference with an explicit lifetime
// &'a mut i32      //a mutable reference with an explicit lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    }
    return y;
}
