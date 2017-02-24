use std::collections::HashMap;

static NUCLEOTIDES: &'static str = "ACGT";

pub fn count(nucleotide: char, input: &str) -> Result<usize, char> {
    let valid_input = |x: char| {NUCLEOTIDES.contains(x)};
    if valid_input(nucleotide) && input.chars().all(valid_input) {
        Ok(input.chars().filter(|&c| c == nucleotide).count())    
    }
    else {
        Err(nucleotide)    
    }
    
} 

pub fn nucleotide_counts(input: &str) -> Result<HashMap<char, usize>, char> {
    let mut map: HashMap<char, usize> = NUCLEOTIDES.chars().map(|c| (c, 0)).collect();
    for nucleotide in input.chars() {
        if let Some(n) = map.get_mut(&nucleotide) {
            *n += 1;
        } else {
            return Err(nucleotide);
        }
    }
    Ok(map)
}
