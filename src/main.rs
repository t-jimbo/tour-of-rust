mod binary_search;
mod encrypt;
mod fizz_buzz;
mod sort;

fn main() {
    fizz_buzz::fizz_buzz(15);

    let found = binary_search::binary_search(3, &[1, 2, 3, 4, 5]);
    println!("{}", if found { "Yes" } else { "No" });

    let found = binary_search::binary_search(7, &[0, 2, 3, 5, 6, 8, 9, 10]);
    println!("{}", if found { "Yes" } else { "No" });

    let converted = encrypt::convert("20050308");
    println!("{}", converted);

    let (encrypted, p, q, d) = encrypt::rsa_decrypt(2773, 17, 1453);
    println!("word is {}. (p, q, d) = ({}, {}, {})", encrypted, p, q, d);
}
