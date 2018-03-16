fn main() {
    let number: usize = 50;

    //println!("The {}th fibonacci number is: {}\n - Calculated using a recursive algorithm.", number, recursive_fibonacci(number));
    println!("The {}th fibonacci number is: {}\n - Calculated using a iterative algorithm.", number, iterative_fibonacci(number));
}

fn recursive_fibonacci(number: usize) -> usize {
    if number == 1 || number == 2{
        return 1
    } else {
        return recursive_fibonacci(number - 1) + recursive_fibonacci(number - 2)
    }
}

fn iterative_fibonacci(number: usize) -> usize {
    let mut fib_numbers = vec![1, 1];

    for n in 3..(number+1) {
        let next_number: usize = fib_numbers[n - 2] + fib_numbers[n - 3];
        fib_numbers.push(next_number);
    }
    return fib_numbers[number - 1]
}
