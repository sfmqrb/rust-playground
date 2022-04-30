
// Language: rust
// do something fun


fn main() {
    // integer addition
    let sum = 5 + 10;
    // integer subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;

    // integration function for printing
    println!("{} {} {} {} {}", sum, difference, product, quotient, remainder);
    let mut tup: (i32, f64, u8) = (500, 6.4, 1);
}

fn print_hello() {
    println!("Hello, world!");
}

fn add_two(a: i32) -> i32 {
    a + 2
}

fn add_three(a: i32) -> i32 {
    add_two(a) + 1
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

fn pentagonal(n: u32) -> u32 {
    n * (3 * n - 1) / 2
}

fn calc_integration(f: fn(f64) -> f64, a: f64, b: f64, n: u32) -> f64 {
    let h = (b - a) / n as f64;
    let mut sum = 0.0;
    for i in 0..n {
        sum += f(a + i as f64 * h);
    }
    sum * h
}

fn calc_parallel_integration(f: fn(f64) -> f64, a: f64, b: f64, n: u32) -> f64 {
    let h = (b - a) / n as f64;
    let mut sum = 0.0;
    let mut threads = vec![];
    for i in 0..n {
        let thread = std::thread::spawn(move || {
            f(a + i as f64 * h)
        });
        threads.push(thread);
    }
    for thread in threads {
        sum += thread.join().unwrap();
    }
    sum * h
}