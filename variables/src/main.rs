fn main() {
    // let x = 5;
    // println!("The value of x is: {}", x);
    // let x = "six";
    // println!("The value of x is: {}", x);
    // const SUBSCRIBER_COUNT: u32 = 100_000;
    // println!("The value of x is: {}", SUBSCRIBER_COUNT);
    //     let tup = ("lets Get Rusty", 100_000);
    //     println!("The channel name {}", tup.0);

    //     let error_codes = [200, 404, 500];
    //     let not_found = error_codes[1];

    let sum = my_function(5, 7);
    println!("Multiplication is {}", sum)
}

fn my_function(x: i32, y: i32) -> i32 {
    // println!("Another function {}", x * y)
    x * y
}
