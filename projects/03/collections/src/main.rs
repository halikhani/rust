use std::collections::HashMap;
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    for i in &v {
        println!("{}", i);
    }


    let mut v2 = vec![1, 2, 3, 4, 5];
    for i in &mut v2 {
        *i += 1;
        println!("{}", i);
    }


    enum SpreadsheetCell{
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => println!("Int: {}", i),
            SpreadsheetCell::Float(f) => println!("Float: {}", f),
            SpreadsheetCell::Text(t) => println!("Text: {}", t),
        }
    }

    // let mut s = String::new();
    let s2 = "initial contents".to_string();
    // OR
    let s3 = String::from("initial contents");
    println!("s2: {}", s2);
    println!("s3: {}", s3);

    let mut s4 = String::from("foo");
    let s5 = "bar";
    s4.push_str(s5);
    println!("s4: {}", s4);
    println!("s5: {}", s5); // s5 is still valid here since push_str takes a reference to s5, not a copy


    let mut s6 = String::from("lo");
    s6.push('l'); // gets a char literal
    println!("s6: {}", s6);

    // using +
    let s7 = String::from("Hello, ");
    let s8 = String::from("world");
    let s9 = s7 + &s8; // the compiler can coerce the &String to a &str since it implements the Deref trait
    println!("s9: {}", s9);
    println!("s8: {}", s8); // stil alive, but s7 is no longer valid since it was moved into s9


    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    // let tic_tac_toe = tic + "-" + &tac + "-" + &toe;
    // OR
    let tic_tac_toe = format!("{}-{}-{}", tic, tac, toe);
    println!("tic_tac_toe: {}", tic_tac_toe);


    println!("--------------------------------");
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("s: {}", s);

    for c in s.chars() {
        println!("{}", c);
    }
    for b in s.bytes() {
        println!("{}", b);
    }


    // hashmaps
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores: {:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // copied() is used to convert the Option<&i32> to an Option<i32>
    // unwrap_or(0) is used to return 0 if the Option is None
    println!("score: {}", score);


    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Note: for owned values like String, the key and value will be moved into the hashmap, and the original value will be invalidated
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!


    let mut scores_2 = HashMap::new();

    scores_2.insert(String::from("Blue"), 10);
    scores_2.insert(String::from("Blue"), 50);

    // score for insert to same key will overwrite the previous value
    scores_2.entry(String::from("Yellow")).or_insert(60);
    scores_2.entry(String::from("Blue")).or_insert(70);
    println!("scores_2: {:?}", scores_2);


    // updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;

    }

    println!("{map:?}");
}
