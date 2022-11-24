use std::io;

fn main() {
    loop {
        let mut input= String::new();
        io::stdin().read_line(&mut input).expect("No number");
        let input = input.trim().parse().expect("Unparsable");
        println!("{}",fib(input));

    }
}
fn fib (n:u32) -> i64{
    let mut p1 = 0;
    let mut p2 = 0;
    let mut cur = 1;
    for _ in 1..n {
        p1 = p2;
        p2 = cur;
        cur = p1 + p2;
    }
    cur
}
