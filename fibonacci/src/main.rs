use std::collections::HashMap;

fn main() {
    let mut memo = HashMap::new();
    let n = 55;

    let result = fib(n, &mut memo);
    println!("{}", result);
}

fn fib(n: i64, memo: &mut HashMap<i64, i64>) -> i64 {
    if n <= 2 {
        return 1;
    }
    if let Some(&result) = memo.get(&n) {
        return result;
    }
    let value = fib(n - 1, memo) + fib(n - 2, memo);
    memo.insert(n, value);
    value
}
