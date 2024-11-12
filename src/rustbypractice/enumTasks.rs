#[test]
fn test6_1()
{
    enum Number {
        Zero,
        One,
        Two,
    }

    enum Number1 {
        Zero = 0,
        One,
        Two,
    }

    // C-like enum
    enum Number2 {
        Zero = 0,
        One = 1,
        Two = 2,
    }


    // An enum variant can be converted to a integer by `as`
    assert_eq!(Number::One as i32, Number1::One as i32);
    assert_eq!(Number1::One as i32, Number2::One as i32);

    println!("Success!");
}

#[test]
fn test6_2()
{
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg1 = Message::Move{x: 1, y: 2}; // Instantiating with x = 1, y = 2
    let msg2 = Message::Write("hello, world!".to_string()); // Instantiating with "hello, world!"

    println!("Success!");
}
#[test]
fn test6_3()
{
    // Fill in the blank and fix the error
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::Move{x: 1, y: 2};

    if let Message::Move{x,y} = msg {
        assert_eq!(x, 1);
        assert_eq!(y, 2);
    } else {
        panic!("NEVER LET THIS RUN！");
    }

    println!("Success!");
}
#[test]
fn test6_4()
{

    // Fill in the blank and fix the errors
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }
    fn show_message(msg: Message) {
        match msg {
            Message::Quit => println!("quit"),
            Message::Move { x, y } => println!("coordinates: x={}, y={}", x, y),
            Message::Write(text) => println!("message: {}", text),
            Message::ChangeColor(r, g, b) => println!("color: red={}, green={}, blue={}", r, g, b),
        }
    }
}
#[test]
fn test6_5()
{
    // Fill in the blank to make the `println` work.
    // Also add some code to prevent the `panic` from running.
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let Some(n) = six {
        println!("{}", n);

        println!("Success!");

    }

    else {
        panic!("NEVER LET THIS RUN！");
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
}