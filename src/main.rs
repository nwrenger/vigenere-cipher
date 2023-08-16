use std::{io, vec};

fn main() {
    println!("Encode Message(1), Decode Message(2)");
    let answer = get_input();

    let encode = match answer.trim() {
        "1" => true,
        "2" => false,
        _ => {
            println!("Wrong Input");
            return;
        }
    };

    println!("Input Message:");
    let message = get_input();
    let (result, offset) = caesar(&message, encode);
    if encode {
        println!("Encoded Message:")
    } else {
        println!("Decoded Message:")
    }
    print!("{}", result);
    println!("With offset: {:?}", offset)
}

fn caesar(string: &str, encode: bool) -> (String, Vec<usize>) {
    let alphabet: Vec<char> = ('a'..='z').collect();
    let mut result = String::with_capacity(string.len());
    let mut offset_list: Vec<usize> = vec![];

    for c in string.chars() {
        if c.is_alphabetic() {
            let lowercase = c.to_lowercase().next().unwrap();
            let index = (lowercase as u8 - b'a') as usize;
            println!("Offset for {c}: ");
            let offset: usize = get_input().trim().parse().unwrap();
            offset_list.push(offset);
            let new_index = if encode {
                (index + offset) % 26
            } else {
                (index + 26 - (offset % 26)) % 26
            };

            let new_char: char = alphabet[new_index];
            if c.is_uppercase() {
                result.push(new_char.to_ascii_uppercase());
            } else {
                result.push(new_char);
            }
        } else {
            result.push(c);
        }
    }

    (result, offset_list)
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}
