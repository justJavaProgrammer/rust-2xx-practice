// https://practice.course.rs/ownership/borrowing.html

#[test]
fn test5_2_1()
{
    let x = 5;
    // Fill the blank
    let p = &x;

    println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}
#[test]
fn test5_2_2()
{
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(5, *y);

    println!("Success!");
}
#[test]
fn test5_2_3()
{
    let mut s = String::from("hello, ");
    borrow_object(&s);
    println!("Success!");
    fn borrow_object(s: &String) {}
}
#[test]
fn test5_2_4()
{
    let mut s = String::from("hello, ");

    push_str(&mut s);

    println!("Success!");

    fn push_str(s: &mut String) {
        s.push_str("world")
    }
}
#[test]
fn test5_2_5()
{
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let mut p = Box::new(&mut s);

    p.push_str("world");

    println!("Success!");
}
#[test]
fn test5_2_6()
{
    let c = '中';

    let r1 = &c;
    // Fill the blank，dont change other code
    let ref r2 = c;

    assert_eq!(*r1, *r2);

    // Check the equality of the two address strings
    assert_eq!(get_addr(r1), get_addr(r2));

    println!("Success!");


    // Get memory address string
    fn get_addr(r: &char) -> String {
        format!("{:p}", r)
    }
}
#[test]
fn test5_2_7()
{
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}", r2);

    println!("Success!");
}
#[test]
fn test5_2_8()
{
    // Fix error by modifying this line
    let mut s = String::from("hello, ");

    borrow_object(&mut s);

    println!("Success!");

    fn borrow_object(s: &mut String) {}
}
#[test]
fn test5_2_9()
{
    // This code has no errors!
    let mut s = String::from("hello, ");

    borrow_object(&s);

    s.push_str("world");

    println!("Success!");

    fn borrow_object(s: &String) {}
}
#[test]
fn test5_2_10()
{
    // Comment one line to make it work
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");

    //println!("{}",r1);
}
#[test]
fn test5_2_11()
{
    let mut s = String::from("hello, ");
    let r1 = &mut s;
    //let r2 = &mut s;
    println!("{}",r1)
    // Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
    // You can't use r1 and r2 at the same time
}