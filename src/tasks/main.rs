fn main() {
    let size = 5;
    let mut result = String::new();

    for i in 0..size {
        result.push_str(&" ".repeat(size - i - 1));
        result.push_str(&"* ".repeat(i + 1).trim_end());
        result.push('\n');
    }
    for i in (0..size - 1).rev() {
        result.push_str(&" ".repeat(size - i - 1));
        result.push_str(&"* ".repeat(i + 1).trim_end());
        result.push('\n');
    }

    print!("{}", result);
}
