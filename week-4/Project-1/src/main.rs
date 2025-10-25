use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the value for a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the value for b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the value for c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");


    let d:f32 = b.powf(2.0) - 4.0*a*c;
    println!("The discriminant is {}", d);

    if d > 0.0{
        let x1 = (-b + d.sqrt()) / (2.0*a);
        let x2 = (-b - d.sqrt()) / (2.0*a);
        println!("There are two distinct roots. Where x1 = {} and x2 = {}", x1, x2);
    }
    else if d == 0.0{
        let root2:f32 = -b/(2.0*a);
        println!("There is exactly one real root. Where x = {}", root2);
    }
    else{
        let disc:f32 = -d;
        let real:f32 = -b / (2.0*a);
        let imag:f32 = disc.sqrt() / (2.0*a);
        println!("There are no real roots. Where x1 = {} + {}i, x2 = {} - {}i", real, imag, real, imag);
    }

}
