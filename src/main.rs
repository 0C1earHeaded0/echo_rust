use std::{char, env};

#[repr(u8)]
enum Keys {
    CancelNewLine = 1,
    AllowEscape = 2,
}

fn main() {
    let mut argv: Vec<String> = env::args().collect();
    process_argv(&mut argv);
    // for i in 1..argv.len() {
    //     print!("{} ", argv[i]);
    //     if i == argv.len() - 1 {
    //         println!();
    //     }
    // }
}

fn process_argv(argv: &mut Vec<String>) {
    let mut key_bit_mask: u8 = 0;
    for i in 1..argv.len() {
        let non_formatted_output = argv[i..].join(" ");
        if argv[i].chars().nth(0) == Some('-') {
            for j in 1..argv[i].len() {
                let chr: char = match argv[i].chars().nth(j) {
                    Some(char) => char,
                    None => '\0',
                };

                match recognize_key(chr) {
                    Some(key) => {
                        change_key_mask(&mut key_bit_mask, key);
                    },
                    None => println!("{}", non_formatted_output),
                }
            }
            argv[i] = String::new();
        }
    }

    if (key_bit_mask & Keys::AllowEscape as u8) != 0 {
        let mut str = argv[1..].join(" ");
        str = str.replace(r"\n", "\n");
        str = str.replace(r"\t", "\t");
        println!("{}", str.trim_start());
    } else if (key_bit_mask & Keys::CancelNewLine as u8) != 0 {
        print!("{}", argv[1..].join(" ").trim_start());
    } else {
        println!("{}", argv[1..].join(" ").trim());
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
