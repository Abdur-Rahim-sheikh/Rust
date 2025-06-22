use unicode_segmentation::UnicodeSegmentation;

fn main(){
    let hello = String::from("সালাম");
    // in char format
    // Scaler values
    // [স, া, ল, া, ম]
    for c in hello.chars(){
        println!("{c}")
    } 

    // in byte format
    // [224, 166, 184, 224, 166, 190, 224, 166, 178, 224, 166, 190, 224, 166, 174]
    let c = hello.bytes().nth(0);
    println!("{c:?}");

    // in grapheme clusters
    for g in hello.graphemes(true){
        println!("{g}")
    }

}



// fn main() {
//     let a = [1,2,3];
//     let mut v:Vec<i32> = Vec::new();
//     v.push(1);
//     v.push(2);
//     let mut v2 = vec![1,2,3];
//     let third = &v2[2];
//     println!("The third element is {third}");

//     let second = v2.get(4).unwrap_or(&-1);
//     println!("The second value is {second:#?}");

//     for i in &mut v2{
//         *i += 50;
//     }
//     for i in &v2{
//         println!("{i}")
//     }
// }
