use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("Document.txt").expect("create failed");
     let mut file2 = OpenOptions::new().append(true).open("Document.txt").expect("cannot open file");
     
     let mut input = String::new();
     println!("\nEnter text: ");
     std::io::stdin().read_line(&mut input).expect("Failed to read");
     let text:String = input.trim().parse().expect("Invalid input");


     file.write_all(text.as_bytes()).expect("write failed");

}
