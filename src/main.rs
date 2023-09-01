use std::io;

fn main() {
    println!(
        "Running in Version: {}, starting...",
        env!("CARGO_PKG_VERSION")
    );
    println!("Disclaimer: This only works correctly with the English alphabet!");
    println!("Encode Message(1), Decode Message(2):");

    let answer = get_input();
    let encode = match answer.trim() {
        "1" => true,
        "2" => false,
        _ => {
            println!("Invalid input. Please enter 1 or 2.");
            return;
        }
    };

    println!("Cipher:");
    let message = get_input();
    println!("Key:");
    let key = get_input();

    let result: String = message
        .chars()
        .zip(key.trim().chars().cycle())
        .map(|x| vigenere(x, encode))
        .collect();

    if encode {
        println!("Encoded Message:")
    } else {
        println!("Decoded Message:")
    }

    print!("{}", result);
    println!("Press enter or close the Terminal");
    get_input();
}

fn vigenere(x: (char, char), encode: bool) -> char {
    let (c, k) = x;
    let shift = (k.to_ascii_lowercase() as u8 - b'a') as i32;
    let operation = if encode { shift } else { -shift };

    match c {
        'a'..='z' => (((c as u8 - b'a') as i32 + operation).rem_euclid(26) as u8 + b'a') as char,
        'A'..='Z' => (((c as u8 - b'A') as i32 + operation).rem_euclid(26) as u8 + b'A') as char,
        _ => c,
    }
}

fn get_input() -> String {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            return input;
        }
        println!("Wrong input");
    }
}
