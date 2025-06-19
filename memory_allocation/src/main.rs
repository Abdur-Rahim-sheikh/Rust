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
}

fn takes_ownership(str: String) {
    println!("{}", str);
}
