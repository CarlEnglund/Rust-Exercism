pub fn reply(message: &str) -> String {
    let all_caps = |input: &str| input.chars().all(|x| x.to_uppercase().collect::<String>() == x.to_string());
    let question_mark = |input: &str| input.find('?') == Some(input.len()-1);

    if message.len() == 0 {
        return "Fine. Be that way!".to_string();
    }
    if all_caps(message) {
       return "Whoa, chill out!".to_string();
    }
    if question_mark(message) {
       return "Sure.".to_string();
    }
    "Whatever.".to_string()
}
