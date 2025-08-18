mod help;

use crate::help::show_help;
use std::{char, env};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_key_mask() -> Result<(), String> {
        let mut test_key_bit_mask: u8 = 0;

        change_key_mask(&mut test_key_bit_mask, Keys::AllowEscape);
        change_key_mask(&mut test_key_bit_mask, Keys::CancelNewLine);

        if test_key_bit_mask == 3 {
            Ok(())
        } else {
            Err(String::from(format!("Expected 2 given {}", test_key_bit_mask)))
        }
    }

    #[test]
    fn test_print_output() {
        let strv: Vec<String> = Vec::from(["echo".to_owned(), "abc\\n".to_owned(), "def".to_owned(), "jhi".to_owned()]);

        assert_eq!(print_output(&strv, 0), "abc\\n def jhi\n");
        assert_eq!(print_output(&strv, 1), "abc\\n def jhi");
        assert_eq!(print_output(&strv, 2), "abc\n def jhi\n");
        assert_eq!(print_output(&strv, 3), "abc\n def jhi");
    }
}


#[repr(u8)]
enum Keys {
    CancelNewLine = 0,
    AllowEscape = 1,
}

const BASE: u8 = 2;

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

fn print_output(argv: &Vec<String>, key_bit_mask: u8) -> String {
    let mut str = String::from(argv[1..].join(" ").trim());

    if (key_bit_mask & BASE.pow(Keys::AllowEscape as u32)) != 0 {
        str = str.replace(r"\n", "\n");
        str = str.replace(r"\t", "\t");
    } 
    
    if (key_bit_mask & BASE.pow(Keys::CancelNewLine as u32)) != 0 {
        print!("{}", str);
    } else {
        println!("{}", str);
        str.push('\n');
    }

    return str;
}

fn change_key_mask(mask: &mut u8, key: Keys) {
    match key {
        Keys::CancelNewLine => *mask |= BASE.pow(Keys::CancelNewLine as u32),
        Keys::AllowEscape => *mask |= BASE.pow(Keys::AllowEscape as u32),
    }
}

fn recognize_key(key: char) -> Option<Keys> {
    match key {
        'n' => Option::Some(Keys::CancelNewLine),
        'e' => Option::Some(Keys::AllowEscape),
        _ => Option::None,
    }
}
