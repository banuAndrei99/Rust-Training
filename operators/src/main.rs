
const MEANING_OF_LIFE: u8 = 42;  // type must be explicit; no fixed address
static mut Z:i32 = 123;  // type must be explicit; adding mut to it makes it unsafe


fn operators(){
    // arithmetic
    let mut a = 2 + 3 * 4;
    a += 1;  // does not support -- or ++

    println!("remainder of {} / {} = {}", a, 3, (a%3));

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed = {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {} to py = {}", b, b_cubed, b, b_to_pi);

    // bitwise
    let c = 1 | 2;  // | = OR; & = AND; ^ = XOR; ! = NOR
                        // 01 OR 10 = 11 = 3
    println!("1|2 = {}", c);
    let two_to_10 = 1 << 10;  // shifted 10 times
    println!("2^10 = {}", two_to_10);

    // logical
    let pi_less_than_4 = std::f64::consts::PI < 4.0; // true
    // < <= >= > == !=
    println!("pi is smaller than 4 = {}", pi_less_than_4);
}


fn main() {
    operators();
    println!("{}", MEANING_OF_LIFE);
    unsafe{
        Z = 777;
        println!("{}", Z);
    }
}
