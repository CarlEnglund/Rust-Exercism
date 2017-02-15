pub fn square_of_sum(num: i32) -> i32 {
  let summary: i32 = (0 .. num + 1).sum();
  summary.pow(2)
}

pub fn sum_of_squares(num: i32) -> i32 {
  let summary: i32 = (0 .. num + 1).map(|n| n.pow(2)).sum();
  summary
}

pub fn difference(num: i32) -> i32 {
  square_of_sum(num) - sum_of_squares(num)  
}
