/// https://practice.course.rs/variables.html

#[test]
// Fix the error below with least amount of modification to the code
fn test3_1()
{
    let x = 5;
    assert_eq!(x, 5);
    println!("Success!");
}
#[test]
// Fill the blanks in the code to make it compile
fn test3_2()
{
    let mut x =1;
    x+=2;
    assert_eq!(x, 3);
    println!("Success!");
}
#[test]
// Fix the error below with least amount of modification
fn test3_3()
{
    let x: i32 = 10;
    let y: i32;
    {
        y= 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);
}

#[test]
fn main() {
    let x = define_x();
    println!("{}, world", x);
}

fn define_x() -> String {
    let x = "hello".to_string();
    return x
}
#[test]
// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
fn test3_5()
{
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}

#[test]
fn test3_6()
{
    let x: i32 = 7;
    let x = x + 3;

    let _y = 4;
    let _y = "I can also be bound to text!";

    println!("Success! Value of x is: {}", x);
    println!("Success! Value of y is: {}", _y);
}

#[test]
fn test3_7_1()
{
    let x = 1;
    for i in 1..10
    {
        println!("{}", x*i);
    }
}
#[test]
fn test3_7_2()
{
    let x:i32 = 2;
    println!("Value of x is: {}", x.pow(10))
}
#[test]
fn test3_8()
{
    let (mut x, y) = (1, 2);
    x += 2;
    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}
#[test]
fn test3_9()
{
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3,2]);

    println!("Success!");
}