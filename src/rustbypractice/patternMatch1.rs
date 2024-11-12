// https://practice.course.rs/pattern-match/match-iflet.html

#[test]
fn test8_1()
{
    enum Direction {
        East,
        West,
        North,
        South,
    }
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North  => { // Matching South or North here
            println!("South or North");
        },
        _ => println!("West"),
    };
}
#[test]
fn test8_2()
{
    let boolean = true;

    // Fill the blank with a match expression:
    //
    // boolean = true => binary = 1
    // boolean = false =>  binary = 0
    let binary = boolean as i32;

    assert_eq!(binary, 1);

    println!("Success!");
}
#[test]
fn test8_3()
{
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msgs = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }

    println!("Success!");

    fn show_message(msg: Message) {
        match msg {
            Message::Move{x: a, y: b} => { // match  Message::Move
                assert_eq!(a, 1);
                assert_eq!(b, 3);
            },
            Message::ChangeColor(r, g, b) => {
                assert_eq!(g, 255);
                assert_eq!(b, 0);
            }
            _ => println!("no data in these variants")
        }
    }
}
#[test]
fn test8_4()
{
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];

    // Check if each character is an uppercase letter
    for ab in alphabets {
        assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9'));
    }

    println!("Success!");
}
#[test]
fn test8_5()
{
    #[derive(PartialEq)]
    enum MyEnum {
        Foo,
        Bar
    }

    let mut count = 0;

    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    for e in v {
        if e == MyEnum::Foo { // Fix the error by changing only this line
            count += 1;
        }
    }
    assert_eq!(count, 2);

    println!("Success!");
}
#[test]
fn test8_6()
{
    let o = Some(7);

    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);
        println!("Success!");
    }
}
#[test]
fn test8_7()
{
    enum Foo {
        Bar(u8)
    }

    let a = Foo::Bar(1);

    if let Foo::Bar(i) = a {
        println!("foobar holds the value: {}", i);

        println!("Success!");
    }
}
#[test]
fn test8_8()
{
    enum Foo {
        Bar,
        Baz,
        Qux(u32)
    }
    let a = Foo::Qux(10);
    match a
    {
        a @ Foo::Bar => println!("Bar"),
        a @ Foo::Baz => println!("Baz"),
        _ => println!("other")
    }
}
#[test]
fn test8_9()
{
    let age = Some(30);
    if let Some(age) = age { // Create a new variable with the same name as previous `age`
        assert_eq!(age, 30);
    } // The new variable `age` goes out of scope here

    match age {
        // Match can also introduce a new shadowed variable
        Some(age) =>  println!("age is a new variable, it's value is {}",age),
        _ => ()
    }
}