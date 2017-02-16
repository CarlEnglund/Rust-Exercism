use std::collections::BTreeSet;

pub fn sum_of_multiples(num: u64, factors: &[u64]) -> u64 {
  let mut storage_vector: BTreeSet<u64> = BTreeSet::new();

  for &f in factors {
     let mut multiplier = 2;
     let mut factor: u64 = f;
     while factor < num {
      storage_vector.insert(factor);
      factor = f * multiplier;
      multiplier += 1;
     }
   } 
    storage_vector.iter().sum() 
}
