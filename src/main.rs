fn main() {
    let name = "taro";
    hello(name)
}

fn hello(name: &str) {
    println!("Hello, {}", name)
}
