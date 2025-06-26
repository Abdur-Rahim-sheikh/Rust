use std::fs::File;
use std::io::ErrorKind;
fn main() {
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(e) => match e.kind(){
            ErrorKind::NotFound => {
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                }
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        }
           
        
    };
}
