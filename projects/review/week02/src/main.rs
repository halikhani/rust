// Week 2 optional task: arrays, loops and vectors.
// Run the tests with `cargo test`, and `cargo run` to try things in `main`.

// TODO: Return the largest number in `nums` using a `for` loop.
// (No `.iter().max()` or other iterator methods.)
//
// Hints:
// - Keep the largest value seen so far in a `let mut` variable.
// - What should it start as? Think about an array of all negative numbers
//   before choosing `0`.
// - `for n in nums` gives you each number in the array.
fn largest(nums: [i32; 5]) -> i32 {
    let mut largest = nums[0];
    for n in nums {
        if n > largest {
            largest = n
        }
    }
    largest
}

// TODO: Return every even number from 1 up to and including `n`, in order.
// For example, `evens(7)` returns `[2, 4, 6]`.
//
// Hints:
// - Start with an empty vector and `push` onto it, like `fizzbuzz_loop`.
// - `..` vs `..=`: which one includes `n`?
// - A number is even when `i % 2 == 0`.
fn evens(n: u32) -> Vec<u32> {
    let mut res = Vec::new();
    for e in 1..=n {
        if e % 2 == 0 {
            res.push(e);
        }
    }
    res
}

fn main() {
    // You can optionally experiment here.
    println!("{}", largest([3, 9, 2, 7, 5]));
    println!("{:?}", evens(10));
}

// Don't edit the tests; make them pass.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_in_the_middle() {
        assert_eq!(largest([3, 9, 2, 7, 5]), 9);
    }

    #[test]
    fn largest_at_the_ends() {
        assert_eq!(largest([10, 1, 2, 3, 4]), 10);
        assert_eq!(largest([1, 2, 3, 4, 10]), 10);
    }

    #[test]
    fn largest_all_negative() {
        assert_eq!(largest([-8, -3, -5, -9, -4]), -3);
    }

    #[test]
    fn largest_all_equal() {
        assert_eq!(largest([7, 7, 7, 7, 7]), 7);
    }

    #[test]
    fn evens_up_to_ten() {
        assert_eq!(evens(10), vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn evens_odd_limit() {
        assert_eq!(evens(7), vec![2, 4, 6]);
    }

    #[test]
    fn evens_small_inputs() {
        assert!(evens(0).is_empty());
        assert!(evens(1).is_empty());
        assert_eq!(evens(2), vec![2]);
    }
}
