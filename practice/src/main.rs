use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the first number: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let x:f32 = input1.trim().parse().expect("Not a valid integer");

    println!("Enter the second number: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let y:f32 = input2.trim().parse().expect("Not a valid integer");

    println!("List of operations: ");
    println!("(1) Add");
    println!("(2) Subtract");
    println!("(3) Multiply");
    println!("(4) Divide");
    println!("Select the number of your desired operation: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let op:i32 = input3.trim().parse().expect("Not a valid integer");

    if op == 1{
        let a:f32 = x + y;
        println!("The result = {}", a);
    }
    else if op == 2{
        let s:f32 = x - y;
        println!("The result = {}", s);
    }
    else if op == 3{
        let m:f32 = x * y;
        println!("The result = {}", m);
    }
    else if op == 4{
        let d:f32 = x / y;
        println!("The result = {}", d);
    }
    else{println!("Invalid input.");}


}
