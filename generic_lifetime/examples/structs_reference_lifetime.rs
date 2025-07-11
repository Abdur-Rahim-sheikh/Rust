struct ImportExcerpt<'a> {
    part: &'a str,
}

impl<'b> ImportExcerpt<'b> {
    fn return_part(&self, announcement: &str) -> &str {
        println!("Attention please {announcement}");
        return self.part;
    }
}
fn main() {
    let novel = String::from("Call me Ismail. Some years ago....");
    let first_sentence = novel.split('.').next().expect("Could not find");

    let i = ImportExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part);
}
