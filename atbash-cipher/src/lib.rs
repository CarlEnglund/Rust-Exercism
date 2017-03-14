use std::ascii::AsciiExt;

fn ascii(ch: char) -> u8 {
    ch as u8
}

fn get_correct_char(ch: char) -> char {
    if ch.is_digit(10) {
       return ch;
    }
        
    (ascii('z') - ascii(ch) + ascii('a')) as char
}

fn cipherengine(input: &str) -> Vec<String> {
    input.to_lowercase().chars().filter(|&c| c.is_ascii())
         .filter(|&c| c.is_alphanumeric())
         .map(|c| get_correct_char(c))
         .collect::<Vec<char>>()
         .chunks(5)
         .map(|slice| slice.iter().cloned().collect::<String>())
         .collect::<Vec<String>>()
}
pub fn encode(input: &str) -> String {
         cipherengine(input).join(" ")
}

pub fn decode(input: &str) -> String {
         cipherengine(input).join("")
}


