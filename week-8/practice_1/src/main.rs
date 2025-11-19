fn main() {
   let vector1: Vec<i64> = Vec::new();

   println!("\nThe length of Vec::new is {}",vector1.len() );

   let vector1 = vec!["Mary", "Len", "Omori", "Khaleel", "Suzy"];
   println!("\nThe length of vec macro is: {}",vector1.len() );
}
