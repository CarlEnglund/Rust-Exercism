pub fn raindrops(number: i32) -> String {
   let is_pling = |number| number % 3 == 0; 
   let is_plang = |number| number % 5 == 0; 
   let is_plong = |number| number % 7 == 0; 
   let mut return_string = String::new();

   if is_pling(number) {
       return_string.push_str("Pling");
   }
   if is_plang(number) {
       return_string.push_str("Plang");
   }
   if is_plong(number) {
       return_string.push_str("Plong");
   }
   if return_string.is_empty() {
       let s = format!("{}", number);
       return_string.push_str(&s);
   }
   return_string
}
