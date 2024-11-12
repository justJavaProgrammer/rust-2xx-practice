// https://practice.course.rs/basic-types/char-bool-unit.html
#[test]
fn test_4_1()
{
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4);

    println!("Success!");
}
#[test]
fn test_4()
{
    // Make it work
    fn main() {
        let c1 = '中';
        print_char(c1);
    }
    fn print_char(c : char) {
        println!("{}", c);
    }
}
#[test]
fn test_4_3()
{
    let _f: bool = false;

    let _t = true;
    if !_f {
        println!("Success!");
    }
}
#[test]
fn test_4_4()
{
    let f = true;
    let t = true && false;
    assert_eq!(t, !f);

    println!("Success!");
}
#[test]
fn test_4_5()
{
    let _v: () = ();

    let v = ();
    assert_eq!(v, implicitly_ret_unit());

    println!("Success!");

    fn implicitly_ret_unit() {
        println!("I will return a ()");
    }

    // Don't use this one
    fn explicitly_ret_unit() -> () {
        println!("I will return a ()");
    }
}
#[test]
fn test_4_6()
{
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);
    println!("Success!");
}