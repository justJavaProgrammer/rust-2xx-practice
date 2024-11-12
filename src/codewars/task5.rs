use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plusMinus(arr: &[i32]) {

    let mut positive = 0;
    let mut negative = 0;
    let mut zero = 0;

    for number in arr {
        if *number > 0 {
            positive += 1;
        } else if *number < 0 {
            negative += 1;
        } else if *number == 0 {
            zero += 1;
        }
    }

    println!("{:.6}", positive as f64 / arr.len() as f64);
    println!("{:.6}", negative as f64 / arr.len() as f64);
    println!("{:.6}", zero as f64 / arr.len() as f64);

}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plusMinus(&arr);
}
