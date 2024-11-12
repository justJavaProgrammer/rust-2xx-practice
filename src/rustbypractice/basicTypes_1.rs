/// https://practice.course.rs/basic-types/numbers.html
#[test]
fn test4_1()
{
    let x: u32 = 5;
    let mut y: u32 = 5;

    y = x;

    let z: i8 = 10; // Type of z ?

    println!("Success!");
}
#[test]
fn test4_2()
{
    let v: u16 = 38_u8 as u16;
    println!("Success!");
}
#[test]
fn test4_3()
{
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));
    println!("Success!");
}
// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String
{
    format!("{}", std::any::type_name::<T>())
}
#[test]
fn test4_4()
{
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);
    println!("Success!");
}

#[test]
fn test4_5()
{
    let v1 = 25u8 + 4;
    let v2 = u8::checked_add(251, 4).unwrap();
    println!("{},{}",v1,v2);
}
#[test]
fn test4_6()
{
    let v = 024 + 0xff + 0o77 + 0b1111111;
    assert!(v == 1597);
    println!("Success!");
}
#[test]
fn test4_7()
{
    let x = 000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z = 0.0f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
    fn type_of<T>(_: &T) -> String
    {
        format!("{}", std::any::type_name::<T>())
    }
}
#[test]
fn test4_8()
{
    let epsilon = 0.0000f64;
    assert!((0.1+0.2)-0.3 < epsilon);
    println!("Success!");
}
#[test]
fn test4_9()
{
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
fn test4_10()
{
    // Fill the blanks
    use std::ops::{Range, RangeInclusive};
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
    println!("Success!");
}

#[test]
fn test4_11()
{
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1u8 - 1 == 0);

    assert!(3 * 50 == 150);
    assert!((9.6 / 3.2)-3.0 < 0.0001);
    assert!(24 % 5 == 4);
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}