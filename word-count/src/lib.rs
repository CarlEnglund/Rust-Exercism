use std::collections::HashMap;

pub fn word_count(s: &str) -> HashMap<String, u32> {
    let mut word_map: HashMap<String, u32> = HashMap::new();

    let lower = s.to_lowercase();
    let slice: &str = lower.as_ref();

    for word in slice.split(|c: char| !c.is_alphanumeric()).filter(|s| !s.is_empty()) {
        *word_map.entry(word.to_string()).or_insert(0) += 1;
    }

    word_map
    
}
