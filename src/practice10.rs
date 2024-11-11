
fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    let shift = (n % len + len) % len;
    let split_index = len - shift;
    let (left, right) = s.split_at(split_index as usize);

    return format!("{}{}", right, left)
}

#[test]
fn test() {
    let s = "abcdefgh";
    let shifts = [
        (0, "abcdefgh"),
        (8, "abcdefgh"),
        (-8, "abcdefgh"),
        (1, "habcdefg"),
        (2, "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10, "cdefghab"),
    ];

    for (n, exp) in shifts.iter() {
        assert_eq!(rotate(s.to_string(), *n), exp.to_string());
    }
}