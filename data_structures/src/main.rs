use crate::Color::Red;

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point
}

fn structures() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("point p is at ({}, {})", p.x, p.y);

    let p2 = Point { x: 5.0, y: 10.0 };

    let my_line = Line {start: p, end: p2};
}

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),  //tuple
    Cmyk{cyan: u8, magenta: u8, yellow: u8, black: u8}  // struct
}

fn enums() {
    let c: Color = Color::Cmyk{ cyan: 0, magenta: 0, yellow: 4, black: 255};
    match c {
        Color::Red => println!("red"),
        Color::Blue => println!("blue"),
        Color::Green => println!("green"),
        Color:: RgbColor(0, 0, 0) => println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb({}, {}, {})", r, g, b),
        Color::Cmyk{cyan:_, magenta:_, yellow:_, black: 255} => println!("black"),
        Color::Cmyk{cyan, magenta, yellow, black} => println!("cmyk({}, {}, {}, {})", cyan, magenta, yellow, black),

    }
}


// 32 bits occupied either by a integer or a float
union IntOrFloat {
    i: i32,
    f: f32
}

fn unions() {
    let mut iof = IntOrFloat { i: 123 };
    iof.i = 234;

    unsafe{
        let value = iof.i;  // this is unsafe because we don't know if we have and integer of floating point number inside the union
        println!("{}", value);
    }
    process_union(IntOrFloat{i: 5});

}

fn process_union(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => {println!("Meaning of life value")},
            IntOrFloat{ f } => {  // this treats any value as a f32 -> reinterpret cast
                println!("value = {}", f);
            }
        }
    }
}

fn option_t() {
    let x = 3.0;
    let y = 2.0;

    // Option -> Some(v) | None
    let result =
        if y != 0.0 { Some(x/y) } else { None };

    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("Cannot divide by 0")
    }


    // let checks if the value from the right side can be assigned to the left side
    if let Some(z) = result {
        println!("result of {}/{} = {}", x, y, z)
    }

}

fn arrays() {
    let mut a:[i32; 5] = [1, 2, 3, 4, 5];  // arrays cannot be resized
    println!("a has {} elements, first is {}", a.len(), a[0]);

    a[0] = 321;
    println!("{:?}", a);

    if a != [1, 2, 3, 4, 5] {
        println!("does not match");
    }

    let b = [1; 10]; // b.len() = 10
    for i in 0..b.len() {
        println!("{}", b[i]);
    }

    let matrix: [[f32; 3]; 2] = [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];
    println!("{:?}", matrix);

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if i == j {
                println!("On diagonal: {}", matrix[i][j])
            }
        }
    }
}

fn use_slice(slice: &mut [i32]) {
    println!("first elem = {}, len = {}", slice[0], slice.len());
    println!("slice is {:?}", slice);

    slice[0] = 1234;
}

fn slices() {
    let mut data = [1, 2, 3, 4, 5];

    use_slice(&mut data[1..4]);
    use_slice(&mut data);
    println!("{:?}", data)
}

fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    return (x+y, x*y);
}

fn tuples() {
    let tuple = sum_and_product(4, 2);
    println!("tuple = {:?}", tuple);
    println!("tuple.0 = {}", tuple.0);
    let (sum, prod) = sum_and_product(4, 2);
    println!("sum = {}, prod = {}", sum, prod);
}

fn how_many(x: i32) -> &'static str {
    match x {
        0 => "no ",
        1 | 2 => "one or two ",
        12 => "a dozen ",
        z @ 8...11 => "lots of ",  // z is the name for the range
        _ if (x % 2 == 0) => "some ",
        _ => "a few "
    }
}

fn pattern_matching() {
    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many(x));
    }

    let point = (3, 4);
    match point {
        (0, 0) => println!("origin"),
        (0, _y) => println!("on _x axis"),
        (_x, 0) => println!("on _y axis"),
        (x, y) => println!("{} {}", x, y)
    }
}

fn main() {
    structures();
    enums();
    unions();
    option_t();
    arrays();
    slices();
    tuples();
    pattern_matching();
}
