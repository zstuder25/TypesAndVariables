#![allow(dead_code)]

mod sh;
use std::mem;

const MEANING_OF_LIFE:u8 = 42; // no fixed address

static mut Z:i32 = 123;

fn fundamental_data_types(){
    // unsigned 0.. 255
    let a:u8 = 123; // 8bits
    println!("a = {}", a);


    let mut b:i8 = 0; // mutable
    println!("b = {}", b);
    b = 42;
    println!("b = {}", b);

    let mut c = 123456789; // 32-bit signed i32
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {} after modification", c);

    // i8 u8 i16 u16 i32 u32 i64 u64
    let z:isize = 123; // isize/usize
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8);

    let d:char = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    let e:f32 = 2.5; // double-precision, 8 bytes or 64 bits, f64 unless specified f32
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    // true false
    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
    let f = 4>0; // true
    println!("f = {}, size = {} bytes", f, mem::size_of_val(&f));
}

fn operators(){
    // arithmetic
    let mut a = 2+3*4;
    println!("{}", a);
    a = a + 1; // -- ++
    a -= 2; // a = a -2
            // -= += *= /= %=
    println!("remainder of {} / {} = {}", a, 3, (a%3));

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    // bitwise
    let c = 1 | 2; // | OR & AND ^ XOR ! NOR
                        // 01 OR 10 = 11 == 3_10
    println!("1|2 = {}", c);

    let two_to_10 = 1 << 10; // >> shift bits to the right
    println!("2^10 = {}", two_to_10);

    //logical
    let pi_less_4 = std::f64::consts::PI < 4.0; //true
    // > <= >= ==
}

fn scope_and_shadowing()
{
    let a = 123;
    {
        let b = 456;
        println!("b = {}", b);

        let a = 777; //shadowing
        println!("inner a = {}", a);
    }
    println!("outside a = {}", a)
}

fn if_statements()
{
    let temp = 15;

    if temp > 30
    {
        println!("really hot outside");
    }
    else if temp < 10
    {
        println!("really cold");
    }
    else
    {
        println!("temperature is ok");
    }

    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("today is {}", day);

    println!("it is {}",
        if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"OK"});

    println!("it is {}",
        if temp > 20 {
            if temp > 30 { "very hot" } else { "hot" }
        }
        else if temp < 10 {"cold"} else {"ok"});
}

fn while_and_loop()
{
    let mut x  = 1;

    while x < 1000
    {
        x *= 2;

        if x == 64 { continue; } // this interrupts the block and takes it back to beginning of loop

        println!("x = {}", x);
    }

    let mut y = 1;
    loop // while true
    {
        y *= 2;
        println!("y = {}", y);

        if y == 1<<10 { break; } // will break out of the loop
    }
}

fn for_loop()
{
    for x in 1..11 // 1 to 10 inclusive
    {
        if x == 3 { continue; } // no more 3
        if x == 8 { break; } // no more 8, 9, 10
        println!("x = {}", x)
    }

    for (pos, y) in (30..41).enumerate()
    {
        println!("{}: {}", pos, y);
    }
}

fn match_statement()
{
    let country_code = 999; // 1 999

    let country = match country_code  // Rust switch statement
    {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1...999 => "Unknown", // 1..99 is end exclusive, 1...999 is inclusive
        _ => "Invalid"
    };

    println!("the country with code {} is {}", country_code, country)
}

struct Point
{
    x:  f64,
    y: f64
}

struct Line
{
    start: Point,
    end: Point
}

fn structures()
{
    let p = Point { x: 3.0, y: 4.0 };
    println!("point p is at ({}, {})", p.x, p.y);

    let p2 = Point { x: 5.0, y: 10.0 };
    let myline = Line { start: p, end: p2};
}

enum Color
{
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), // tuple
    CmykColor {cyan:u8, magenta:u8, yellow:u8, black:u8} // struct
}

fn enums()
{
    // let c:Color =  Color::Red;
    // let c:Color =  Color::RgbColor(0,0,1);
    let c:Color =  Color::CmykColor {cyan: 0, magenta: 128, yellow: 0, black: 255};

    match c // Must have all cases for enums or have _ default
    {
        Color::Red => println!("r"),
        Color::Green => println!("r"),
        Color::Blue => println!("r"),
        Color::RgbColor(0,0,0)
            | Color::CmykColor {cyan:  _, magenta: _, yellow: _, black: 255} => println!("black"),
        Color::RgbColor(r,g,b) => println!("rgb({}, {},{})", r, g, b),
        _ => ()
    };
}

// 32 bits
union IntOrFloat
{
    i: i32,
    f: f32
}

fn  process_value(iof: IntOrFloat)
{
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => {
                println!("meaning of life value")
            }
            IntOrFloat{ f} => {
                println!("value  = {}",  f)
            }
        }
    }
}

fn unions()
{
    let mut iof = IntOrFloat { i:  123 };
    iof.i = 234;
    let value = unsafe { iof.i };
    println!("iof.i = {}", value);
    process_value(IntOrFloat{ i: 5})
}

fn main() {
    // --- Types and Variables ---
    // fundamental_data_types()
    // operators();
    // scope_and_shadowing()
    // println!("{}", MEANING_OF_LIFE);
    // unsafe {
    //     println!("{}", Z) // must do this because its mutable
    // }\

    // --- Stacks and Heaps ---
    // sh::stacks_and_heaps();

    // --- Control Flow ---
    // if_statements()
    // while_and_loop()
    // for_loop()
    // match_statement()

    // --- Data Structures ---
    // structures()
    // enums()
    unions()
}
