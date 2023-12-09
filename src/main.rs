/*
 * main function
 */
fn main() {
    let name = "taro";
    hello(name)
}

// print hello
fn hello(name: &str) {
    println!("Hello, {}", name)
}
