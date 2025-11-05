use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
  loop{
    println!("Cafe menu:");
    println!("Code  Item    Price(N) 
            \nT    Tea      8
            \nC    Coffee   1200
            \nS    Sandwich  2000
            \nJ     Juice    1500");

    println!("\nEnter the item code:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let mut i = input1.trim();

    println!("Enter the quantity: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let q:f32 = input2.trim().parse().expect("Not a valid integer");

    
    if i == "T"{
        let mut t:f32 = 800.0 * q;
    println!("Your total cost is {}", t);
    }
    else if  i == "C"{
      let mut t:f32 = 1200.0 * q;
    println!("Your total cost is {}", t);
    }
    else if i == "S"{
    let mut t:f32 = 2000.0 * q;
    println!("Your total cost is {}", t);    }
    else if i == "J"{
        let mut t:f32 = 1500.0 * q;
    println!("Your total cost is {}", t);
    }


    if t >= 5000.0{
        let t = t * (5-100)/100.0;
    }
    let mut input3 = String::new();
    println!("type exit to stop loop: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let m = input3.trim();

    if m == "exit"{
        break
    }
}
 



}
