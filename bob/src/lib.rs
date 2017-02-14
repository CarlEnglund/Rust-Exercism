pub fn reply(message: &str) -> String {
    let all_caps = |input: &str| input.chars().all(|x| x.to_uppercase().collect::<String>() == x.to_string());
    let question_mark = |input: &str| input.find('?') == Some(input.len()-1);

    if message.len() == 0 {
        let s = "Fine. Be that way!".to_string();
        return s;
    }

    if all_caps(message) {
       let s = "Whoa, chill out!".to_string();
       return s;
    }

    if question_mark(message) {
       let s = "Sure.".to_string();
       return s;
    }

    let s = "Whatever.".to_string();
    s
        
}
