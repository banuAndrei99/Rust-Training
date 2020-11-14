#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::mem;

struct Point{
    x: f64,
    y: f64
}


fn origin() -> Point{
    Point{x: 0.0, y: 0.0}
}


pub fn stack_and_heap(){
    let p1 = origin();
    // Box construct allocates memory on the heap
    let p2 = Box::new(origin()); // p2 is a pointer to the location where the Point is actually stored

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));  // 16 bytes = the whole memory allocated
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));  // 8 bytes = the memory allocated for the address pointer

    let p3 = *p2;  // * -> putting the value back on the stack --> unBox-ing
}


fn main() {
    let a: u8 = 123;  // u = unsigned, 8 bits --> 0 - 255
    println!("a = {}", a);
    // a = 456;  let bidding => in immutable variables, all variables are immutable by default


    // u = unsigned, 0 --> 2^(N-1)
    // i = signed, -2^(N-1) --> 2^(N-1)-1
    let mut b: i8 = 123; // mut keyword will explicitly make the variable mutable
    println!("b = {} before", b);
    b = -4;
    println!("b = {} after", b);

    let c = 123456789; // type inferred --> i32
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));

    // u8, u16, u32, u64, i8, i16, ...

    // usize isize --> size of processor bitness

    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);  // type inferred --> usize
    println!("z = {}, takes up {}, {}-bit OS", z, size_of_z, size_of_z * 8);

    let d: char = 'x';
    println!("d = {} is a char, takes up {} bytes", d, mem::size_of_val(&d));

    // f32, f64 --> standardize(IEEE754) floating point numbers
    // f32, f64 --> all signed!

    let e = 2.5; // type inferred to f64(default type for non-whole numbers)
    println!("e = {}, takes up {} bytes", e, mem::size_of_val(&e));

    let g = false; // type inferred to bool
    println!("g = {}, takes up {} bytes", g, mem::size_of_val(&g));

    stack_and_heap();
}
