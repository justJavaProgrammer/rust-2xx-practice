#[test]
fn main() {
    let triangles = 7;
    for i in 0..triangles
    {
        for j in 0..(i+1)
        {
            println!("{}{}", " ".repeat(triangles - j - 1), "*".repeat(2 * j + 1));
        }
    }
}