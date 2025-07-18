struct Point<T, U> {
    x: T,
    y: U,
}
impl <T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        return Point {
            x: self.x,
            y: other.y,
        };
    }
}
fn main(){
    let p1 = Point { x: 10, y: 10.3 };
    let p2 = Point {x: "hello", y: 'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

}