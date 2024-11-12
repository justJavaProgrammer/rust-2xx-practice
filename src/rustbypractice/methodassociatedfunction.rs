// https://practice.course.rs/method.html

#[test]
fn test9_1()
{
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // Complete the area method which return the area of a Rectangle.
        fn area(&self) -> i32
        {
            (self.width*self.height) as i32
        }
    }

    let rect1 = Rectangle { width: 30, height: 50 };

    assert_eq!(rect1.area(), 1500);

    println!("Success!");
}
#[test]
fn test9_2()
{
    #[derive(Debug)]
    struct TrafficLight {
        color: String,
    }

    impl TrafficLight {
        pub fn show_state(&self) {
            println!("the current state is {}", self.color);
        }
    }
    let light = TrafficLight {
        color: "red".to_owned(),
    };
    // Don't take the ownership of `light` here.
    light.show_state();
    // ... Otherwise, there will be an error below
    println!("{:?}", light);
}
#[test]
fn test9_3()
{
    struct TrafficLight {
        color: String,
    }

    impl TrafficLight {
        // Using `Self` to fill in the blank.
        pub fn show_state(&self)  {
            println!("the current state is {}", self.color);
        }

        // Fill in the blank, DON'T use any variants of `Self`.
        pub fn change_state(s : &mut TrafficLight) {
            s.color = "green".to_string()
        }
    }
    println!("Success!");
}
#[test]
fn test9_4()
{
    #[derive(Debug)]
    struct TrafficLight {
        color: String,
    }

    impl TrafficLight {
        pub fn new() -> Self {
            Self {
                color: String::from("red")
            }
        }

        pub fn get_state(&self) -> &str {
            &self.color
        }
    }
    let light = TrafficLight::new();
    assert_eq!(light.get_state(), "red");

    println!("Success!");
}
#[test]
fn test9_5()
{
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // Using multiple `impl` blocks to rewrite the code below.
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
    println!("Success!");
}
#[test]
fn test9_6()
{
    #[derive(Debug)]
    enum TrafficLightColor {
        Red,
        Yellow,
        Green,
    }

    // Implement TrafficLightColor with a method.
    impl TrafficLightColor {
        pub fn color(&self) -> &str {
            match self {
                TrafficLightColor::Red => "red",
                TrafficLightColor::Yellow => "yellow",
                TrafficLightColor::Green => "green",
            }
        }
    }
    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}",c);
}