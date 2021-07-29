#[derive(Debug)]
struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    fn new(w: f32, h: f32) -> Rectangle {
        Rectangle {
            width: w,
            height: h,
        }
    }
}

#[derive(Debug)]
struct Triangle {
    base: f32,
    height: f32,
}

impl Triangle {
    fn new(b: f32, h: f32) -> Triangle {
        Triangle {
            base: b,
            height: h,
        }
    }
}

#[derive(Debug)]
struct Circle {
    radius: f32,
}

impl Circle {
    fn new(r: f32) -> Circle {
        Circle {
            radius: r,
        }
    }
}

trait ComputeArea {
    fn get_area(&self) -> f32;
}

impl ComputeArea for Rectangle {
    fn get_area(&self) -> f32 {
        self.width * self.height
    }
}

impl ComputeArea for Triangle {
    fn get_area(&self) -> f32 {
        (self.base * self.height) / 2_f32
    }
}

impl ComputeArea for Circle {
    fn get_area(&self) -> f32 {
        3.14 * self.radius.powf(2_f32)
    }
}

fn get_area<T: ComputeArea>(t: &T) -> f32 {
    t.get_area()
}

fn main() {
    let rectangle = Rectangle::new(12_f32, 10_f32);
    let triangle = Triangle::new(8_f32, 6_f32);
    let circle = Circle::new(12_f32);

    println!("rectangle: {:?}, area: {}", rectangle, get_area(&rectangle));
    println!("triangle: {:?}, area: {}", triangle, get_area(&triangle));
    println!("circle: {:?}, area: {}", circle, get_area(&circle));
}
