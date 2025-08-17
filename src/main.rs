mod help;

use crate::help::show_help;
use std::{char, env};

#[repr(u8)]
enum Keys {
    CancelNewLine = 1,
    AllowEscape = 2,
}

fn main() {
    let mut argv: Vec<String> = env::args().collect();

    if argv.iter().len() == 1 {
        show_help();
    }

    process_argv(&mut argv);
}

fn process_argv(argv: &mut Vec<String>) {
    let mut key_bit_mask: u8 = 0;

    'outer_loop: for i in 1..argv.len() {
        if argv[i].chars().nth(0) == Some('-') {
            for j in 1..argv[i].len() {
                let chr: char = match argv[i].chars().nth(j) {
                    Some(char) => char,
                    None => '\0',
                };

                match recognize_key(chr) {
                    Some(key) => {
                        change_key_mask(&mut key_bit_mask, key);
                    }
                    None => {
                        break 'outer_loop
                    },
                }
            }
            argv[i] = String::new();
        }
    }

    print_output(argv, key_bit_mask);
}

fn print_output(argv: &Vec<String>, key_bit_mask: u8) {
    let mut str = String::from(argv[1..].join(" ").trim());

    if (key_bit_mask & Keys::AllowEscape as u8) != 0 {
        str = str.replace(r"\n", "\n");
        str = str.replace(r"\t", "\t");
    } 
    
    if (key_bit_mask & Keys::CancelNewLine as u8) != 0 {
        print!("{}", str);
    } else {
        println!("{}", str);
    }
}

fn change_key_mask(mask: &mut u8, key: Keys) {
    match key {
        Keys::CancelNewLine => *mask |= Keys::CancelNewLine as u8,
        Keys::AllowEscape => *mask |= Keys::AllowEscape as u8,
    }
}

fn recognize_key(key: char) -> Option<Keys> {
    match key {
        'n' => Option::Some(Keys::CancelNewLine),
        'e' => Option::Some(Keys::AllowEscape),
        _ => Option::None,
    }
}
