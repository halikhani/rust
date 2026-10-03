fn main() {
    println!("{}", lib_hello::greeting());
}

fn fizzbuzz(n: u32) -> Vec<String> {
    (1..=n)
        .map(|i| match (i % 3, i % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            (_, _) => i.to_string(),
    }).collect()
}

// TODO: Implement `fizzbuzz_loop` using only a `for` loop and `if` / `else if` / `else`
// (no `match`, `map` or `collect`). For each number from 1 to `n` (inclusive):
// - multiple of both 3 and 5 -> "FizzBuzz"
// - multiple of 3            -> "Fizz"
// - multiple of 5            -> "Buzz"
// - otherwise                -> the number itself, as text
//
// Hints:
// - `let mut out = Vec::new();` creates an empty vector you can add to.
// - `out.push(String::from("Fizz"));` adds a string; `i.to_string()` turns a number into text.
// - Think about the order of your checks: which condition must come first?
// - The function must return `out`. Watch the last line (expression vs statement).
fn fizzbuzz_loop(n: u32) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for i in 1..n + 1 {
        if i % 3 == 0 && i % 5 == 0 {
            out.push("FizzBuzz".to_string());
        }
        else if i % 3 == 0 {
            out.push("Fizz".to_string());
        }
        else if i % 5 == 0 {
            out.push("Buzz".to_string());
        }
        else {
            out.push(i.to_string());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*; // brings all the functions and macros from the outer scope into the test module

    #[test]
    fn fizzbuzz_5() {
        assert_eq!(
            fizzbuzz(5),
            vec!["1", "2", "Fizz", "4", "Buzz"] // vec![] is a macro that creates a vector
                .into_iter() // into_iter() is a method that converts the vector into an iterator so we can iterate over it with map or collect
                .map(String::from) // for each item in the vector, convert it to a string using the String::from method
                .collect::<Vec<_>>(),
        );
    }

    #[test]
    fn fizzbuzz_15_has_fizzbuzz() {
        let out = fizzbuzz(15);
        assert_eq!(out[14], "FizzBuzz");  // 15th element (index 14)
    }

    // Tests for `fizzbuzz_loop`. Don't edit these; make them pass.

    #[test]
    fn loop_zero_is_empty() {
        assert!(fizzbuzz_loop(0).is_empty());
    }

    #[test]
    fn loop_first_five() {
        assert_eq!(fizzbuzz_loop(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
    }

    #[test]
    fn loop_fizz_buzz_fizzbuzz() {
        let out = fizzbuzz_loop(15);
        assert_eq!(out.len(), 15);
        assert_eq!(out[2], "Fizz"); // 3
        assert_eq!(out[4], "Buzz"); // 5
        assert_eq!(out[8], "Fizz"); // 9
        assert_eq!(out[9], "Buzz"); // 10
        assert_eq!(out[14], "FizzBuzz"); // 15
    }

    #[test]
    fn loop_matches_iterator_version() {
        assert_eq!(fizzbuzz_loop(100), fizzbuzz(100));
    }
}
