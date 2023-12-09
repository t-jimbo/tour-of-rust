pub fn fizz_buzz(num: i32) {
    for i in 1..=num {
        if i % 15 == 0 {
            println!("fizz buzz");
            continue;
        }
        if i % 3 == 0 {
            println!("fizz");
            continue;
        }
        if i % 5 == 0 {
            println!("buzz");
            continue;
        }
        println!("{}", i);
    }
}
