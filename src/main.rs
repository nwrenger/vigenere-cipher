use std::io;

const ALPHABET_SIZE: u8 = 26;

fn main() {
    println!(
        "Running in Version: {}, starting...",
        env!("CARGO_PKG_VERSION")
    );
    println!("Disclaimer: This only works correctly with the English alphabet!");
    println!("Encode Message(1), Decode Message(2)");

    let answer = get_input();
    let encode = match answer.trim() {
        "1" => true,
        "2" => false,
        _ => {
            println!("Wrong Input");
            main();
            return;
        }
    };

    println!("Input Message:");
    let message = get_input();
    println!("Input Key (only alphabet, numbers etc. won't do anything!):");
    let key = get_input();

    let result = vigenere(&message, &key, encode);

    if encode {
        println!("Encoded Message:")
    } else {
        println!("Decoded Message:")
    }

    print!("{}", result);
    println!("Press enter or close the Terminal");
    get_input();
}

fn vigenere(string: &str, key: &str, encode: bool) -> String {
    let mut result = String::with_capacity(string.len());
    let mut nth = 0;

    for c in string.chars() {
        if c.is_alphabetic() {
            let index = c as u8 - to_ascii_u8(c);
            let offset = key.chars().cycle().nth(nth).map(to_ascii_u8).unwrap_or(0);
            let shifted = (if encode {
                (index + offset) % ALPHABET_SIZE
            } else {
                (index + ALPHABET_SIZE - (offset % ALPHABET_SIZE)) % ALPHABET_SIZE
            } + to_ascii_u8(c)) as char;

            result.push(shifted);

            nth = (nth + 1) % key.len();
        } else {
            result.push(c);
        }
    }

    result
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

fn to_ascii_u8(c: char) -> u8 {
    if c.is_ascii_uppercase() {
        b'A'
    } else {
        b'a'
    }
}
