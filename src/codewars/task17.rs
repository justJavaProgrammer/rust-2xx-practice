use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'migratoryBirds' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */
use std::collections::HashMap;

fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut hm:HashMap<i32,i32> = HashMap::new();
    for a in arr.into_iter(){
        *hm.entry(*a).or_insert(0) += 1;
    }
    let max_val = hm.clone().into_iter().max_by(|a,b| a.1.cmp(&b.1)).unwrap().1;
    let hm_v = hm.into_iter().filter(|p| p.1 == max_val).collect::<Vec<_>>();
    hm_v.into_iter().min_by(|a,b| a.0.cmp(&b.0)).unwrap().0
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = migratoryBirds(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
