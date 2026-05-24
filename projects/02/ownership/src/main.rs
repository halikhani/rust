fn main() {
    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // println!("The length of '{}' is {}.", s1, len);

    // // mutable references
    // let mut s = String::from("hello"); // first change s to be mut, NOTE: we only can have one mutable reference to a variable at a time
    // change(&mut s);

    

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{r1}, {r2}");

    // correct way:
    let mut s = String::from("hello1");

    {
        let r1 = &mut s;
        println!("{r1}");
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    r2.push_str(", world2");
    println!("{r2}");


    // let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{r1}, {r2}, and {r3}");

    let mut s2 = String::from("hello");

    let r1 = &s2; // no problem
    let r2 = &s2; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s2; // no problem
    println!("{r3}");

    let mut s3 = String::from("hello world");

    let word = first_word(&s3);

    s3.clear(); // error!

    println!("the first word is: {word}");


}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope and is dropped, so its memory goes away.
  // Danger!


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}