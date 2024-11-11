fn is_palindrome(x: i32) -> bool {
    let (mut rev, mut original) = (0, x);

    while original > 0 {
        rev = (rev * 10) + original % 10;
        original /= 10;
    }

    return rev == x
}

#[test]
fn test() {
    assert_eq!(true, is_palindrome(1));
    assert_eq!(true, is_palindrome(11));
    assert_eq!(true, is_palindrome(1331));

    assert_eq!(false, is_palindrome(21));
    assert_eq!(false, is_palindrome(10));
    assert_eq!(false, is_palindrome(1332));
}