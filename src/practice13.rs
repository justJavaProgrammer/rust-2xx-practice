fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total: u32 = shipments.iter().sum();
    let len = shipments.len() as u32;

    if total % len != 0 {
        return usize::MAX;
    }

    let average = total / len;
    let mut moves = 0;
    let mut imbalance = 0;

    for &shipment in shipments.iter() {
        imbalance += shipment as i32 - average as i32;
        moves += imbalance.abs() as usize;
    }

    moves
}

fn main() {
    let shipments = vec![10, 20, 30, 40];
    println!("Minimum moves: {}", count_permutation(&shipments));
}




// Чи завжди можливо забезпечити однакову кількість грузу?
// Ні, не завжди можливо забезпечити однакову кількість грузу на всіх кораблях. Якщо загальна кількість грузу не ділиться на рівну кількість кораблів, тоді неможливо розподілити груз однаково.


// Сігнатура в іншому випадку
fn count_permutation(shipments: &Vec<u32>) -> Option<usize>


// Функція генерації Vec<u32> які можуть бути розподілені однаково
use rand::Rng;

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut shipments: Vec<u32> = (0..n).map(|_| rng.gen_range(10..100)).collect();
    let total: u32 = shipments.iter().sum();
    let average = total / n as u32;
    let remainder = total % n as u32;

    for i in 0..remainder as usize {
        shipments[i] += 1;
    }

    shipments
}

fn main() {
    let shipments = gen_shipments(20);
    println!("{:?}", shipments);

    match count_permutation(&shipments) {
        usize::MAX => println!("Помилка!."),
        moves => println!("Мінімум кроків: {}", moves),
    }
}