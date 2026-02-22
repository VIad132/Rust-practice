/// https://practice.course.rs/basic-types/intro.html
#[test]
// Remove something to make it work
fn test411() {
    let x: i32 = 5;
    let mut y = 5;

    y = x;

    let z: i32 = 10; // Type of z ?

    println!("Success!");
}
#[test]
// Fill the blank
fn test412() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}
#[test]
// Modify `assert_eq!` to make it work
fn test413() {
    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
#[test]
// Fill the blanks to make it work
fn test414() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");
}
#[test]
// Fix errors and panics to make it work
fn test415() {
    let v1 = 251_u16 + 8;
    let v2 = i16::checked_add(251, 8).unwrap();
    println!("{},{}",v1,v2);
}
#[test]
// Modify `assert!` to make it work
fn test416() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    println!("Success!");
}
#[test]
// Fill the blank to make it work
fn test417() {
    let x: f64 = 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z: f64 = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
#[test]
fn test418() {
    assert!(0.1f32+0.2f32==0.3f32);

    println!("Success!");
}
#[test]
fn test419() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}",c as u8);
    }
}
#[test]
// Fill the blanks
use std::ops::{Range, RangeInclusive};
fn test4110() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}
#[test]
// Fill the blanks and fix the errors
fn test4111() {
    // Integer addition
    assert!(1u32 + 2u32 == 3u32);

    // Integer subtraction
    assert!(1i32 - 2i32 == -1i32);
    assert!(1i8 - 2i8 == -1);

    assert!(3 * 50 == 150);

    assert!(9.6 as f32 / 3.2 as f32 == 3.0 as f32); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
#[test]
// Make it work
use std::mem::size_of_val;
fn test421() {
    let c1: char = 'a';
    assert_eq!(size_of_val(&c1),4);

    let c2: char = '中';
    assert_eq!(size_of_val(&c2),4);

    println!("Success!");
}
#[test]
// Make it work
fn test422() {
    let c1: char = '中';
    print_char(c1);
}

fn print_char(c : char) {
    println!("{}", c);
}
#[test]
// Make println! work
fn test423() {
    let _f: bool = false;

    let t: bool = true;
    if t {
        println!("Success!");
    }
}
#[test]
// Make it work
fn test424() {
    let f: bool= false;
    let t: bool = true && false;
    assert_eq!(t, f);

    println!("Success!");
}
#[test]
// Make it work, don't modify `implicitly_ret_unit` !
fn test425() {
    let _v: () = ();

    let v: (i32, i32)= (2, 3);
    assert_eq!(_v, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}
#[test]
// Modify `4` in assert to make it work
use std::mem::size_of_val;
fn test426() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}
#[test]
fn test431() {
    let x: u32 = 5u32;

    let y: u32 = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z: u32 = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
#[test]
// Make it work with two ways
fn test432() {
    let v: i32 = {
        let mut x: i32 = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);

    println!("Success!");
}
#[test]
fn test433() {
    let v: i32 = {
        let x = 3;
        x
    };

    assert!(v == 3);

    println!("Success!");
}
#[test]
fn test434() {
    let s: i32 = sum(1 , 2);
    assert_eq!(s, 3);

    println!("Success!");
}
fn sum(x: i32, y: i32) -> i32 {
    x + y
}


#[test]
fn test441() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
#[test]
fn test442() {
    print();
}

// Replace i32 with another type
fn print() {
    println!("Success!");
}

// Solve it in two ways
// DON'T let `println!` work
fn test443() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    // Implement this function, don't modify the fn signatures
    panic!()
}

#[test]
fn test444() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };

    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    todo!()
}



#[test]
fn test445() {
    // FILL in the blank
    let b = false;

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}













