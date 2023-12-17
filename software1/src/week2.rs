use std::collections::HashMap;

fn main() {
    println!("Hello, week2!");

    solve(&mut 132);
}

fn asterisk(n: usize, m: usize) -> String {
    let mut res = String::new();
    for _ in 0..m - n {
        res += " ";
    }
    for _ in 0..n {
        res += "*";
    }
    res
}

fn solve(n: &mut usize) {
    let coins = [100, 50, 10, 5, 1];
    let mut counts = coins
        .map(|coin| (coin, 0))
        .into_iter()
        .collect::<HashMap<_, _>>();

    let mut max = 0;

    for coin in coins {
        let count = *n / coin;
        *counts.get_mut(&coin).unwrap() += count;
        *n -= coin * count;
        if count > max {
            max = count;
        }
    }

    for coin in coins {
        println!("{} : {:>3} yen", asterisk(counts[&coin], max), coin);
    }
}
