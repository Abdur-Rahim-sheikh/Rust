fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s; // this is not possible as r1 is still used below, which is in scope
    println!("{}, {}", r1, r2);
    let r3 = &mut s;
    println!("{}", r3); // but this is possible. as no r1, r2 scope is up
}
