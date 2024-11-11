use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut res = Vec::with_capacity(n);

    for i in 0..n {
        let num = rng.gen_range(10..100);
        res.push(num);
    }

    return res;
}

#[test]
fn main() {
    let random_vec = gen_random_vector(20);
    println!("{:?}", random_vec);

    if let Some(min_sum) = min_adjacent_sum(&random_vec) {
        println!("Мінімальна парна сума: (індекс,сума) {:?}", min_sum);
    } else {
        println!("Мінімум 2 елемента має бути в векторі");
    }
}

fn min_adjacent_sum(data: &[i32]) -> Option<(usize, i32)> {
    if data.len() < 2 {
        return None;
    }

    let mut min_sum = data[0] + data[1];
    let mut min_index = 0;

    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    return Some((min_index, min_sum));
}