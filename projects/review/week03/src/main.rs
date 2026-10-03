// Week 3 optional task: ownership, borrowing and slices.
// Run the tests with `cargo test`, and `cargo run` to try things in `main`.

// TODO: Return the total number of bytes in all the strings (sum of `.len()`).
// This version takes the vector BY VALUE: it takes ownership of `words`.
//
// Hints:
// - `let mut total = 0;` then loop over `words` and add each `w.len()`.
// - Before running: after `total_len_owned(words)`, can the caller still use
//   `words`? Why?
fn total_len_owned(words: Vec<String>) -> usize {
    let mut total = 0;
    for w in words {
        total += w.len();
    }
    total
}

// TODO: Same result as `total_len_owned`, but BORROW the strings instead.
//
// Hints:
// - `&[String]` is a slice: a borrowed view of some `String`s. A `&Vec<String>`
//   can be passed where `&[String]` is expected.
// - Looping over `words` here gives `&String`, a reference to each string.
//   `.len()` works on it directly.
// - Before running: after `total_len_borrowed(&words)`, can the caller still
//   use `words`? Why is this different from the owned version?
fn total_len_borrowed(words: &[String]) -> usize {
    let mut total = 0;
    for w in words {
        total += w.len();
    }
    total
}

// TODO: Add "!" to the end of every string, changing the caller's vector in place.
// Nothing is returned: the caller sees the change through the mutable borrow.
//
// Hints:
// - Looping over `words` (a `&mut Vec<String>`) gives `&mut String` for each item.
// - `s.push_str("!")` appends text to a `String` (Book 4.1).
// - What does the caller need to write so that it can pass `&mut words`?
fn shout_all(words: &mut Vec<String>) {
    for w in words {
        w.push('!');
    }
}

fn main() {
    let words = vec![String::from("hi"), String::from("rust")];

    println!("{}", total_len_borrowed(&words));
    println!("{}", total_len_borrowed(&words)); // fine: only borrowed

    println!("{}", total_len_owned(words));

    // PREDICT, then uncomment one line at a time and run `cargo build`.
    // Write down the error code you expect before you look.
    //
    // println!("{:?}", words);              // 1. after it was moved
    // println!("{}", total_len_owned(words)); // 2. moving it a second time
    //
    // let nums = vec![1, 2, 3];
    // let first = &nums;
    // let moved = nums;                      // 3. moving while borrowed...
    // println!("{:?}", first);               //    ...and then using the borrow
}

// Don't edit the tests; make them pass.
#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<String> {
        vec![String::from("hi"), String::from("rust"), String::from("")]
    }

    #[test]
    fn owned_total() {
        assert_eq!(total_len_owned(sample()), 6);
    }

    #[test]
    fn owned_empty() {
        assert_eq!(total_len_owned(Vec::new()), 0);
    }

    #[test]
    fn borrowed_total_and_reuse() {
        let words = sample();
        assert_eq!(total_len_borrowed(&words), 6);
        // `words` is still ours: it was only borrowed.
        assert_eq!(words.len(), 3);
        assert_eq!(total_len_borrowed(&words), 6);
    }

    #[test]
    fn borrowed_part_of_a_vector() {
        let words = sample();
        // A slice can borrow just part of the vector.
        assert_eq!(total_len_borrowed(&words[1..]), 4);
        assert_eq!(total_len_borrowed(&words[..0]), 0);
    }

    #[test]
    fn owned_and_borrowed_agree() {
        let words = sample();
        let borrowed = total_len_borrowed(&words);
        assert_eq!(total_len_owned(words), borrowed);
    }

    #[test]
    fn shout_changes_in_place() {
        let mut words = sample();
        shout_all(&mut words);
        assert_eq!(words, vec!["hi!", "rust!", "!"]);
    }

    #[test]
    fn shout_twice() {
        let mut words = vec![String::from("a")];
        shout_all(&mut words);
        shout_all(&mut words);
        assert_eq!(words, vec!["a!!"]);
    }
}
