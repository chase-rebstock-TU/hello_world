struct Rectangle {
width: f64,
height: f64,
}
struct Circle {
radius: f64, // assume radius > 0
}

impl Circle {
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }
    fn area(&self) -> f64 {
        let pi = 3.14;
         pi * (self.radius * self.radius)
    }
    fn circumference(&self) -> f64 {
        let pi = 3.14;
        2.0 * pi * self.radius
    }
}

impl Rectangle {
fn new(width: f64, height: f64) -> Rectangle {
Rectangle { width, height }
}
fn area(&self) -> f64 {
// TODO: implement
self.width * self.height
}
fn perimeter(&self) -> f64 {
// TODO: implement
2.0 * (self.width + self.height)
}
fn is_square(&self) -> bool {
// TODO: implement
self.width == self.height
}
}
fn main() {
let rect = Rectangle::new(10.0, 5.0);
println!("Area: {}", rect.area());
println!("Perimeter: {}", rect.perimeter());
println!("Is square? {}", rect.is_square());
assert!(Rectangle::new(5.0, 5.0).is_square());
assert!(!Rectangle::new(5.0, 6.0).is_square());
let c = Circle::new(5.0);
println!("Area of circle: {}", c.area());
println!("Circumference of circle: {}", c.circumference());

}