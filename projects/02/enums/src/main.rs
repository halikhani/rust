enum IpAddr {
    V4(String), // or V4(u8, u8, u8, u8) and initialize with (127, 0, 0, 1)
    V6(String),
}


let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));


enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
}

let m = Message::Write(String::from("hello"));
m.call();

enum Option<T> {
    None,
    Some(T),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... more states
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let config_max = Some(3u8);
match config_max {
    Some(max) => println!("Max is {max}"),
    _ => ()
}

// OR
if let Some(max) = config_max {
    println!("Max is {max}");
}


fn describe_state_quarter(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };
    if state.existed_in(1900) {
        Some(String::from("This quarter was minted in 1900"))
    } else {
        Some(String::from("This quarter is relatively recent"))
    }
}

