// 半加算器: 下からの繰り上がりを無視してa + bをする
fn half_adder(a: usize, b: usize) -> (usize, usize) {
    (a & b, a ^ b)
}

// 2倍: ビットシフトするだけ
fn multiple2(a: usize) -> usize {
    a << 1
}

// 全加算器: 半加算器を組み合わせてa + b + cをする
fn full_adder(a: usize, b: usize, c: usize) -> (usize, usize) {
    let (c1, s1) = half_adder(a, b);
    let (c2, s2) = half_adder(s1, c);
    (c1 | c2, s2)
}

// 加算: 全加算器を各桁にかけていく
fn add(mut a: usize, mut b: usize) -> usize {
    let mut c = 0;
    let mut s = 0;
    let mut i = 0;
    while a > 0 || b > 0 {
        let (c_, s_) = full_adder(a & 1, b & 1, c);
        c = c_;
        s |= s_ << i;
        a >>= 1;
        b >>= 1;
        i += 1;
    }
    s |= c << i;
    s
}

// 乗算: 二進数の各桁ごとにビットシフトしたものを加算する
fn multiple(a: usize, mut b: usize) -> usize {
    let mut s = 0;
    let mut i = 0;
    while b > 0 {
        if b & 1 == 1 {
            s = add(s, a << i);
        }
        b >>= 1;
        i += 1;
    }

    s
}

fn q1() {
    let (c, s) = half_adder(0, 0);
    println!("{}, {}", c, s);
    let (c, s) = half_adder(1, 0);
    println!("{}, {}", c, s);
    let (c, s) = half_adder(0, 1);
    println!("{}, {}", c, s);
    let (c, s) = half_adder(1, 1);
    println!("{}, {}", c, s);
}

fn q2() {
    println!("{}", multiple2(7));
    println!("{}", multiple2(377));
}

fn q3() {
    println!("{}", add(4, 15));
    println!("{}", add(2355, 461));
}

fn q4() {
    println!("{}", multiple(6, 7));
    println!("{}", multiple(123, 345));
}

fn main() {
    q1();
    println!();
    q2();
    println!();
    q3();
    println!();
    q4();
}
