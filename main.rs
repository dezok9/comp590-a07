fn main() {
    // Known-answer checks — these will panic if your functions are wrong
    assert_eq!(collatz_length(1), 1, "length of 1 should be 1");
    assert_eq!(collatz_length(6), 9, "length of 6 should be 9");
    assert_eq!(collatz_length(27), 112, "length of 27 should be 112");
    println!("collatz_length checks passed.");
    let answer = longest_collatz(1_000_000);
    assert_eq!(answer, 837799, "longest below 1M should be 837799");
    println!("longest_collatz(1_000_000) = {}", answer);
    println!("All checks passed.");
}

fn collatz_length(n: u64) -> u64 {
    // Base case
    if n == 1 {
        return 1
    }

    // Even
    if n % 2 == 0 {
        return 1 + collatz_length(n / 2)
    }
    // Odd 
    else {
        return 1 + collatz_length(3*n + 1)
    }
}

fn longest_collatz(limit: u64) -> u64 {
    helper(1, limit, 1, 1)
}

fn helper(current: u64, limit: u64, best_num: u64, best_len: u64) -> u64 {
    if current >= limit {
        return best_num;
    }

    let current_len = collatz_length(current);

    if current_len > best_len {
        helper(current + 1, limit, current, current_len)
    } else {
        helper(current + 1, limit, best_num, best_len)
    }
}