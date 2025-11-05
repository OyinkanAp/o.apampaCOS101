use std::io;


fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("\nEnter the Amount: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let p:f32 = input1.trim().parse().expect("Not a valid integer");

    println!("Enter the annual interest rate: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let r:f32 = input2.trim().parse().expect("Not a valid integer");

    println!("Enter the number of years: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let t:f32 = input3.trim().parse().expect("Not a valid integer");


    let a:f32 = p * (1.0 + (r/100.0)).powf(t);
    println!("\nThe Amount due after {} year(s) is {}", t, a);
    let i:f32 = a-p;
    println!("\nThe simple interest is {}", i);

    let mut input4 = String::new();
    println!("Calculate for another borrower? (y/n): ");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    let q = input4.trim();

    while q == "y"{
        println!("Enter the Amount: ");
        io::stdin().read_line(&mut input1).expect("Not a valid string");
        let p:f32 = input1.trim().parse().expect("Not a valid integer");

        println!("Enter the annual interest rate: ");
        io::stdin().read_line(&mut input2).expect("Not a valid string");
        let r:f32 = input2.trim().parse().expect("Not a valid integer");

        println!("Enter the number of years: ");
        io::stdin().read_line(&mut input3).expect("Not a valid string");
        let t:f32 = input3.trim().parse().expect("Not a valid integer");


        let a:f32 = p * (1.0 + (r/100.0)).powf(t);
        println!("\nThe Amount due after {} years is {}", t, a);
        let i:f32 = a-p;
        println!("\nThe simple interest is {}", i);

        let mut input4 = String::new();
        println!("Calculate for another borrower? (y/n): ");
        io::stdin().read_line(&mut input4).expect("Not a valid string");
        let q = input4.trim();
    }
}
