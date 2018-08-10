use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "x: {}, y: {}", self.x, self.y)
     }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{real} +{imag}i",
            real = self.real,
            imag = self.imag
        )
    }
}

fn main() {
    let min_max = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", min_max);
    println!("Debug: {:?}", min_max);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let complex = Complex { real: 3.3, imag: 7.2 };
    println!("Compare complex numbers:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}
