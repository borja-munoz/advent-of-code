use std::collections::HashMap;

// Simple program to show how to solve the computation
// of several fibonacci numbers using dynamic programming.
// Storing the results in a HashMap allows us to
// avoid recomputing values that have been already computed.

// Dynamic programming is useful in situations like this one,
// where we have a problem that can be divided in overlapping
// subproblems.

// In this case, computing the fibonacci number of n, requires
// computing the fibonacci number of (n - 1) and (n - 2).
// When we have to compute multiple numbers, the performance
// gain is higher, but it is useful also for computing a
// single fibonacci number.

fn fibonacci(n: u32, fib: &mut HashMap<u32, u32>) -> u32 {
    if let Some(&f) = fib.get(&n) {
        return f;
    }
    println!("Calculating fibonacci({})...", n);
    if n <= 1 {
        fib.insert(n, n);
        return n;
    }
    let x = fibonacci(n - 1, fib);
    fib.insert(n - 1, x);
    let y = fibonacci(n - 2, fib);
    fib.insert(n - 2, x);
    fib.insert(n, x + y);
    return x + y;
}

fn main() {
    let mut fib = HashMap::new();

    for i in 1..10 {
        println!("Fibonacci({}) = {}", 
            i, 
            fibonacci(i, &mut fib)
        );
    } 
}
