//Trait
trait Area {
    fn area(&self) -> f64;
}

trait Volume {
    fn volume(&self) -> f64;
}



//Struct
struct Square<new> 
{
    side: new,
}

struct Triangle<T=f64> {
    base: T,
    height: T
}

struct Pyramid<T, U> {
    base : T,
    height : U
}



//carré
impl Square<u32> {
    fn new( t: u32) -> Self {
        Square { side : t}
    }
}

impl Square<f64> {
    fn new( t: f64) -> Self {
        Square { side : t}
    }
}

impl Area for Square<u32> {
    fn area(&self) -> f64 {
        (self.side * self.side).into()
    }
}

impl Area for Square<f64> {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Square<String> {
    fn new( t: &str) -> Self {
        let float : f64;
        float = t.parse().unwrap();
        Square { side : (float).to_string(),
    }
}
}

impl Area for Square<String> {
    fn area(&self) -> f64 {
        let tamp : f64 = self.side.parse().unwrap();
        tamp*tamp
    }
}



//triangle
impl Triangle<f64> {
    fn new( b: f64, h: f64) -> Self {
        Triangle {base: b, height: h} 
    }
}

impl Area for Triangle<f64> {
    fn area(&self) -> f64 {
        self.base * self.height / 2.0
    }
}



//pyramide
impl Pyramid<Square<u32>, f64> {
    fn new(base1 : Square<u32>, height1 : f64) -> Self {
        Pyramid { base : base1, height : height1}
    }
}

impl Pyramid<Triangle<f64>, f64> {
    fn new(base1 : Triangle<f64>, height1 : f64) -> Self {
        Pyramid { base : base1, height : height1}
    }
}

impl Volume for Pyramid<Triangle<f64>, f64> {
    fn volume(&self) -> f64 {
        self.base.area() * self.height / 3.0
    }
}

impl Volume for Pyramid<Square<u32>, f64> {
    fn volume(&self) -> f64 {
        self.base.area() * self.height / 3.0
    }
}



fn main() {
    let square = Square::<u32>::new(5);
    let square_float = Square::<f64>::new(5.4);
    let square_string = Square::<String>::new("6");

    println!("square area is {}", square.area());
    println!("square_float area is {}", square_float.area());
    println!("square_string area is {}", square_string.area());

    let triangle = Triangle::new(14.9, 20.1);
    let pyramid_square = Pyramid::<Square<u32>, f64>::new(square, 24.3);
    let pyramid_triangle = Pyramid::<Triangle<f64>, f64>::new(triangle, 24.3);

    println!("pyramid_square volume is {}", pyramid_square.volume());
    println!("pyramid_triangle volume is {}", pyramid_triangle.volume());
}