
/// https://practice.course.rs/ownership/ownership.html
#[test]
fn test5_1()
{
    // Use as many approaches as you can to make it work
    let x = String::from("Hello world");
    let y = &x;
    //let y = x.clone();
    println!("{}, {}",x, y);
}
#[test]
fn test5_2()
{
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2);


    // Only modify the code below!
    fn take_ownership(s: String) -> String
    {
        println!("{}", s);
        s
    }
}
#[test]
fn test5_3()
{
    let s = give_ownership();
    println!("{}", s);
    // Only modify the code below!
    fn give_ownership() -> String {
        let s = String::from("Hello world");
        // Convert String to Vec
        let _s = s.clone().into_bytes();
        s
    }
}
#[test]
fn test5_4()
{
    // Fix the error without removing any code
    let s = String::from("Hello World");

    print_str(&s);

    println!("{}", s);

    fn print_str(s: &String)
    {
        println!("{}",s)
    }
}
#[test]
fn test5_5()
{
    // Don't use clone ,use copy instead
    let s = String::from("hello");
    let x = (1, 2, (), &s);
    let y = (x.0, x.1, x.2, &x.3);
    println!("{:?}, {:?}", x, y);
}
#[test]
fn test5_6()
{
    let mut s = String::from("Hello ");

    let s1 = &mut s;

    s1.push_str("World!");

    println!("Success!");
}
#[test]
fn test5_7()
{
    fn main() {
        let x = Box::new(5);
        let mut y = Box::new(5);      // update this line, don't change other lines!
        *y = 4;
        assert_eq!(*x, 5);

        println!("Success!");
    }
}
#[test]
fn test5_8()
{
    let t = (String::from("hello"), String::from("world"));

    let _s = t.0;

    // Modify this line only, don't use `_s`
    println!("{:?}", t.1);
}
#[test]
fn test5_9()
{
    let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (s1, s2) = (&t.0, &t.1);

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}