use std::fs;

fn main() {
    let mut file = std::fs::File::create("data.txt").expect("Create failed");
    fs::remove_file("data.txt").expect("could not remove file");
    println!("file is removed");
}
