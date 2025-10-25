use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    
    println!("Do you have experience (Yes or No): ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let e = input1.trim();

    println!("Enter your age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let a:u32 = input2.trim().parse().expect("Not a valid integer");


    if e == "Yes" || e == "yes" && a >= 40{
        println!("Your annual incentive will be N1,560,000.");
    }
    else if e == "Yes" || e == "yes" && a >= 30 && a < 40{
        println!("Your annual incentive will be N1,480,000.");
    }
    else if e == "Yes" || e == "yes" && a < 28{
        println!("Your annual incentive will be N1,300,000");
    }
    else if e == "No" || e == "no"{
        println!("Your annual incentive will be N100,000");
    }
    else{
        println!("Invalid answer.");
    }
}
