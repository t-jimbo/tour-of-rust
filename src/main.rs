/*
 * main function
 */
mod fizz_buzz;

fn main() {
    hello("taro");

    fizz_buzz::fizz_buzz();
}

// print hello
fn hello(name: &str) {
    println!("Hello, {}", name)
}
