pub fn encode(plain: &str) -> String {
    transform(plain, true)
}

pub fn decode(cipher: &str) -> String {
    transform(cipher, false)
}

fn transform(input: &str, encode: bool) -> String {
    let mut result = String::new();
    let mut count = 0;

    for ch in input.chars() {
        if ch.is_ascii_alphabetic() {
            let transformed_char = atbash_char(ch);
            result.push(transformed_char);
            count += 1;
            if encode && count % 5 == 0 {
                result.push(' ');
            }
        } else if ch.is_digit(10) {
            result.push(ch);
            if encode && count % 5 == 4 {
                result.push(' ');
            }
        }
    }

    if encode {
        result = result.trim().to_string();
    }

    result
}

fn atbash_char(c: char) -> char {
    if c.is_ascii_lowercase() {
        (b'z' - (c as u8 - b'a')) as char
    } else if c.is_ascii_uppercase() {
        (b'Z' - (c as u8 - b'A')) as char
    } else {
        c
    }
}
