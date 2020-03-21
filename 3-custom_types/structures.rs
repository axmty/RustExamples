#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// A unit struct.
struct Nil;

// A tuple struct.
struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: &Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: Point { x: right_edge, y: bottom_edge }
    } = rect;

    (right_edge - left_edge) * (top_edge - bottom_edge)
}

fn main() {
    // Create and display (debug) a Person structure.
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);

    // Instanciate and access the fields of the tuple struct Point.
    let point: Point = Point { x: 10.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Use struct update syntax to use some fields of another Point.
    let bottom_right = Point { x: 5.2, ..point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Point structure destructuring with 'let' binding.
    let Point { x: top_edge, y: left_edge } = point;
    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Unit struct instanciation.
    let _nil = Nil;

    // Tuple struct instanciation.
    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Tuple struct destructuring.
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    // Activity: calculate a Rectangle area.
    let rect = Rectangle { 
        top_left: Point { x: 2.3, y: 8.8},
        bottom_right: Point { x: 5.3, y: 4.8 }
    };
    println!("{:?} area is {:.1}", rect, rect_area(&rect));
}