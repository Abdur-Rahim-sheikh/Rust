fn main() {
    let mut s1 = String::from("Hello");
    let len = calculate_length(&s1);
    let len2 = update_str(&mut s1);

    println!(
        "The length of {} is {}. after modification {}",
        s1, len, len2
    );
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();
    return length;
}
fn update_str(s: &mut String) -> usize {
    s.push_str(" world!");
    let length = s.len();
    return length;
}
