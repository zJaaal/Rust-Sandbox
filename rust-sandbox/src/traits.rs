trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    height: f64,
    width: f64,
}

struct Triangle {
    base: f64,
    height: f64,
}
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    pub fn new(value: T) -> Self {
        Container { value }
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        use std::f64::consts::PI;

        PI * &self.radius
    }
}
impl Area for Rectangle {
    fn area(&self) -> f64 {
        &self.height * &self.width
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        (&self.base * &self.height) / 2.0
    }
}
//This function will only accept values that implements Area
fn calculate_area<T: Area>(shape: T) -> f64 {
    shape.area()
}
//Another way to write it
// fn calculate_area<T>(shape: T) -> f64
// where T: Area,
// {
//     shape.area()
// }
pub fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 10.0,
        height: 20.0,
    };

    let triangle = Triangle {
        base: 5.0,
        height: 10.0,
    };

    println!("Circle area: {}", calculate_area(circle));
    println!("Rectangle area: {}", calculate_area(rectangle));
    println!("Triangle area: {}", calculate_area(triangle)); //this won't compile if Triangle doesn't implement Area

    //Generic traits
    assert_eq!(Container::new(42).value, 42);
    assert_eq!(Container::new(3.14).value, 3.14);
    assert_eq!(Container::new("Foo").value, "Foo");
    assert_eq!(
        Container::new(String::from("Bar")).value,
        String::from("Bar")
    );
    assert_eq!(Container::new(true).value, true);
    assert_eq!(Container::new(-12).value, -12);
    assert_eq!(Container::new(Some("text")).value, Some("text"));
}
