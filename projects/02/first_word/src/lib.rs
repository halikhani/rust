pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    &s[..]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_word() {
        let s = "Hello world!";
        assert_eq!(first_word(s), "Hello");
    }

    #[test]
    fn first_word_is_subslice_of_input() {
        let s = "Hello world!";
        let word = first_word(s);
        assert_eq!(word, "Hello");
        assert_eq!(word.as_ptr(), s.as_ptr());
    }
}