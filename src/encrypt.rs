use std::collections::HashMap;

fn num_to_char(num: &str) -> &str {
    let hash = vec![
        ("01".to_owned(), "A"),
        ("02".to_owned(), "B"),
        ("03".to_owned(), "C"),
        ("04".to_owned(), "D"),
        ("05".to_owned(), "E"),
        ("06".to_owned(), "F"),
        ("07".to_owned(), "G"),
        ("08".to_owned(), "H"),
        ("09".to_owned(), "I"),
        ("10".to_owned(), "J"),
        ("11".to_owned(), "K"),
        ("12".to_owned(), "L"),
        ("13".to_owned(), "M"),
        ("14".to_owned(), "N"),
        ("15".to_owned(), "O"),
        ("16".to_owned(), "P"),
        ("17".to_owned(), "Q"),
        ("18".to_owned(), "R"),
        ("19".to_owned(), "S"),
        ("20".to_owned(), "T"),
        ("21".to_owned(), "U"),
        ("22".to_owned(), "V"),
        ("23".to_owned(), "W"),
        ("24".to_owned(), "X"),
        ("25".to_owned(), "Y"),
        ("26".to_owned(), "Z"),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

    return hash.get(num).unwrap();
}

pub fn convert(input: &str) -> String {
    let mut s = input.to_string();
    if input.len() % 2 != 0 {
        s = "0".to_string() + input;
    }

    let mut res = String::new();
    for i in (0..s.len()).step_by(2) {
        let num = &s[i..i + 2];
        res = res + num_to_char(num);
    }

    return res;
}

// Find the product of prime numbers by brute force
fn calculate_p_q(n: u32) -> (u32, u32) {
    let mut p = 0;
    let mut q = 0;
    for i in 2..n {
        if n % i == 0 {
            p = i;
            q = n / i;
            break;
        }
    }

    if p == 0 || q == 0 {
        panic!("p and q not found");
    }

    return (p, q);
}

fn calculate_d(e: u32, p: u32, q: u32) -> u32 {
    let mut n = 1;
    while (n * (p - 1) * (q - 1) + 1) % e != 0 {
        n += 1;
    }
    return (n * (p - 1) * (q - 1) + 1) / e;
}

fn calculate_m(c: u32, d: u32, n: u32) -> u32 {
    let mut m = 1;
    for _ in 0..d {
        m = (m * c) % n
    }
    return m;
}

pub fn rsa_decrypt(n: u32, e: u32, c: u32) -> (String, u32, u32, u32) {
    let (p, q) = calculate_p_q(n);
    let d = calculate_d(e, p, q);
    let m = calculate_m(c, d, n);
    let word = convert(&m.to_string());

    return (word, p, q, d);
}
