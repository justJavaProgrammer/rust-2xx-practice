fn invert_the_case(s: String) -> String {
    let mut res = String::new();

    for char in s.chars() {
        if char.is_lowercase() {
            res.push(char.to_uppercase().next().unwrap());
        } else {
            res.push(char.to_lowercase().next().unwrap())
        }
    }

    return res;
}

#[test]
fn main() {
    let input = "Привет, Hello!";
    let output = invert_the_case(input.to_string());
    assert_eq!("пРИВЕТ, hELLO!", output);
}