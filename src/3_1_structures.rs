#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

struct Unit;
struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn rect_area(&self) -> f32 {
        let Rectangle { top_left, bottom_right } = self;
        let width = top_left.x - bottom_right.x;
        let height = top_left.y - bottom_right.y;
        width * height
    }
}

fn square(point: Point, edge_width: f32) -> Rectangle {
    let Point { x: top_left_x, y: top_left_y} = point;
    let bottom_right = Point { x: top_left_x + edge_width, y: top_left_y + edge_width};
    Rectangle { top_left: point, bottom_right: bottom_right }
}

fn rect_area(rectangle: Rectangle) -> f32 {
    let Rectangle { top_left: Point { x: x1, y: y1 }, bottom_right: Point { x: x2, y: y2 } } = rectangle;
    let width = x2 - x1;
    let height = y1 - y2;
    width * height
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };

    println!("point coordinates: ({} {})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point };

    println!("second point: ({} {})", bottom_right.x, bottom_right.y);

    let Point { x: left_edge, y: top_edge } = point; 

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    let _unit = Unit;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("rectangle area: {}", _rectangle.rect_area());
    println!("rectangle area: {:+}", rect_area(_rectangle));

    let x = -0.0;

    println!("{:?}", x); // 打印原始值，可能显示为 "-0.0"
    println!("{:+}", x); // 打印带符号的值，会将 "-0.0" 显示为 "0.0"

    if x == 0.0 {
        println!("{:?}", 0.0); // 如果是 -0.0，则打印为 0.0
    } else {
        println!("{:?}", x); // 否则，打印原始值
    }

    let created_rectangle = square(point, 100.0f32);
    println!("square: {} {} {} {}", created_rectangle.top_left.x, created_rectangle.top_left.y, created_rectangle.bottom_right.x, created_rectangle.bottom_right.y);
}
