fn main() {
    let a = [1,2,3];
    let mut v:Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    let v2 = vec![1,2,3];
    let third = &v2[2];
    println!("The third element is {third}");

    let second = v2.get(4).unwrap_or(&-1);
    println!("The second value is {second:#?}")
}
