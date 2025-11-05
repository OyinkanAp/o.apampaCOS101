use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\nEnter your name: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let name = input1.trim();

    println!("\nEnter your units consumed(kWh): ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let u:f32 = input2.trim().parse().expect("Not a valid integer");


    if  u >= 0.0 && u <= 100.0
    {
        let a:f32 = u * 20.0;
        println!("\nYour rate per unit is N20 and \nYour Monthly electricity bill is N{}", a );
    }
    else if u >= 101.0 && u <= 300.0
    {
        let b:f32 = u * 35.0;
        println!("\nYour rate per unit is N35 and \nYour monthly electricity bill is N{}", b);
    }
    else if u >= 301.0
    {
        let last:f32 = u * 50.0;
        println!("\nYour rate per unit is N50 and \nYour monthly electricity bill is N{}", last);    
    }
    else if u > 500.0{ 
        let d:f32 = last + 5000.0;
        println!("\nYour rate per unit is N50 and \nYour monthly electricity bill is N{}", d);
    }
    else{println!("Invalid input.");}

}
