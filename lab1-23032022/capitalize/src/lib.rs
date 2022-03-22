use std::env;

/// It is impossible to not copy the string as we try to uppercase the first letter of each word.
/// The explanation is well documented in the .to_uppercase() function:
/// not all lower to upper case correspondence is 1:1, some character are converted in more than 1
/// char. So the method returns an iterator, that we can then convert to a string.
pub fn capitalize(s: &str) -> String {
    let mut to_change = true;
    let mut final_string = String::new();
    for c in s.chars() {
        match c {
            ' ' => {
                to_change = true;
                final_string.push_str(" ");
            },
            _c => {
                if to_change {
                    final_string.push_str(_c.to_uppercase().to_string().as_str());
                    to_change = false;
                } else { final_string.push(_c); }
            }
        }
    }
    final_string
}

fn main() {
    let phrase: Vec<String> = env::args().collect();
    if phrase.len() <= 1 {
        println!("Wrong usage. Pass a string as parameter.");
    } else {
        println!("String is now: '{}'", capitalize(phrase[1].as_str()));
    }
}
