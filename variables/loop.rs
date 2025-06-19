fn main() {
    let mut counter = 0;
    // while
    let mut result = loop {
        println!("Again");
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("{}", result);

    // while
    while result > 2 {
        result -= 1;
    }
    let arr = [10, 20, 30, 20];

    // list for
    for val in arr.iter() {
        println!("The value is {}", val);
    }

    // range
    for val in 1..3 {
        println!("The value is {}", val);
    }
}
