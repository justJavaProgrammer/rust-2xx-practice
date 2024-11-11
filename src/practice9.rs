fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for a in 2..n {
        if n % a == 0 {
            return false;
        }
    }
    return true;
}

#[test]
fn main() {
    assert_eq!(false, is_prime(4));
    assert_eq!(false, is_prime(10));
    assert_eq!(false, is_prime(1));

    assert_eq!(true, is_prime(5));
    assert_eq!(true, is_prime(3));
    assert_eq!(true, is_prime(7));
}