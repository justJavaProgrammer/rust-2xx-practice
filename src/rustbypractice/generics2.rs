// https://practice.course.rs/generics-traits/const-generics.html
#[test]
fn test10_1()
{
    struct Array<T, const N: usize> {
        data: [T; N]
    }
    let arrays = [
        Array {
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 3]
        }
    ];

    println!("Success!");
}
#[test]
fn test10_2()
{
    fn print_array<T: std::fmt::Debug>(arr: T) {
        println!("{:?}", arr);
    }
    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array(arr);
}