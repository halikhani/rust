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
    }).collect();
}


#[cfg(test)]
mod tests {
    use super::*; // brings all the functions and macros from the outer scope into the test module

    #[test]
    fn fizzbuzz_5() {
        assert_eq!(
            fizzbuzz(5),
            vec![["1", "2", "Fizz", "4", "Buzz"]] // vec![] is a macro that creates a vector
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

}
