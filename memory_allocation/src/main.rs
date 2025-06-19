fn main() {
    //    ---------- Ownership Rules --------
    //  1. Each value in Rust has a variable that's called it's owner.
    //  2. There can only be one owner at a time.
    //  3. When the owner goes out of scope, the value will be dropped.

    let x = 5;
    let y = x; // primitive type always get copied

    let s1 = String::from("Hello");
    // let s2 = s1; after this line we cannot use s1, as it's ownership moved to s2
    // but if we want to use both
    let s2 = s1.clone();
    println!("{}", s1);
    // takes_ownership(s2); here s2's ownership passed to takes_ownership->str
    takes_ownership(s2.clone());
    println!("{}", s2);

    // slicing
    let mut s = String::from("Hello world!");
    let s2 = "Hello world";
    let hello = &s[..5];
    let world = &s[6..];
    let word = first_word(&s);
    println!("{word}");
}

fn takes_ownership(str: String) {
    println!("{}", str);
}

// slices
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}
