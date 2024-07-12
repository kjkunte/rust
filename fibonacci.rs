use std::collections::HashMap;

fn fibonacci(target: u32) -> u32 {
    fibonacci_memoized(target, &mut HashMap::new())
}
// target is of the type unsigned 32, where as cache is a mutable HashMap
fn fibonacci_memoized(target:u32, cache: &mut HashMap<u32,u32>) -> u32 {

    match target {
        0=>0,
        1=>1,
        // here cache.get(&n) --> gets the memory location of the n
        // .copied get the value stored at the memory address
        n => cache.get(&n).copied().unwrap_or_else(||{
            let result = fibonacci_memoized(n-1, cache) + fibonacci_memoized(n-2,cache);
            cache.insert(n, result);
            println!("{}", result);
            result
        })
    } // here `match` will look for matching the `patterns` => `expression`
}

fn main() {
    fibonacci(10);
}