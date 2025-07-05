struct Point<T> {
    x: T,
    y: T,
}

impl<U> Point<U> {
    fn x(&self) -> &U {
        return &self.x;
    }
}

impl Point<f64> {
    fn y(&self) -> f64 {
        return self.y;
    }
}
fn main(){
    let p = Point { x: 10, y: 20 };
    let a = p.x();
    let p2 = Point { x: 10.5, y: 20.5 };
    let b = p2.x();
    let c = p2.y();
    println!("x: {a}, y: {b}, y: {c}");
}