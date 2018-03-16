fn main() {
    let n: u64 = 20;
    println!("{}", recursive_factorial(n));
}

fn recursive_factorial(n: u64) -> u64 {
    if n == 1 {
        return 1
    } else {
        return n * recursive_factorial(n - 1)
    }
}

fn iterative_factorial(n: u64) -> u64 {
    let mut product: u64 = 1;
    for number in 1..(n+1) {
        product *= number;
    }

    return product
}
