use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

use std::cmp::min;
fn miniMaxSum(arr: &[i32]) {
    let mut vector = arr.to_vec().iter().map(|&x| x as u32).collect::<Vec<_>>();
    vector.sort();
    let max_count_items = min(vector.len() - 1, 4);
    let smallest_sum = vector.iter().take(max_count_items).sum::<u32>();
    let largest_sum = vector.iter().skip(vector.len() - max_count_items).sum::<u32>();
    println!("{smallest_sum} {largest_sum}");
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}
