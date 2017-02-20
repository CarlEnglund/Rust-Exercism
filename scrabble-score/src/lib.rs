use std::collections::HashMap;

pub fn score(letter: &str) -> u32 {
    let scoring = scoring_rules();
    letter.to_lowercase().chars().map(|char| scoring.get(&char).clone())
    .fold(0, |accumulator, value| accumulator + value.unwrap_or(&0)) 
        
}

fn scoring_rules() -> HashMap<char, u32> {
    let mut scoring_rules = HashMap::new();
    scoring_rules.insert('a', 1);
    scoring_rules.insert('e', 1);
    scoring_rules.insert('i', 1);
    scoring_rules.insert('o', 1);
    scoring_rules.insert('u', 1);
    scoring_rules.insert('l', 1);
    scoring_rules.insert('n', 1);
    scoring_rules.insert('r', 1);
    scoring_rules.insert('s', 1);
    scoring_rules.insert('t', 1);
    scoring_rules.insert('d', 2);
    scoring_rules.insert('g', 2);
    scoring_rules.insert('b', 3);
    scoring_rules.insert('c', 3);
    scoring_rules.insert('m', 3);
    scoring_rules.insert('p', 3);
    scoring_rules.insert('f', 4);
    scoring_rules.insert('h', 4);
    scoring_rules.insert('v', 4);
    scoring_rules.insert('w', 4);
    scoring_rules.insert('y', 4);
    scoring_rules.insert('k', 5);
    scoring_rules.insert('j', 8);
    scoring_rules.insert('x', 8);
    scoring_rules.insert('q', 10);
    scoring_rules.insert('z', 10);
    scoring_rules
}
