struct ImportExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ismail. Some years ago....");
    let first_sentence = novel.split('.').next().expect("Could not find");

    let i = ImportExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part);
}
