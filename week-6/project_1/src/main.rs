use std::io;

fn main() {
    let p = "P = Poundo Yam/Edinkaiko Soup      -N3,200";
    let f = "F = Fried Rice & Chicken           -N3,000";
    let a = "A = Amala &Ewedu Soup              -N2,500";
    let e = "E = Eba & Egusi Soup               -N2,000";
    let w = "W = White Rice & Stew              -N2,500";

    let menu  = format!(" \n{} \n{} \n{} \n{} \n{} ", p, f, a, e, w);
    println!("\nMenu                                Price {}", menu);

    let mut input2 = String::new();
    println!("\nPlease type the letter from your desired order: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let food = input2.trim().to_lowercase();

    let mut input = String::new();
    println!("\nEnter the amount you would like to purchase: ");
    io::stdin().read_line(&mut input).expect("Not a valid number");
    let quantity:i32 = input.trim().parse().expect("Not a valid number");


    let price = match food.as_str() {
        "p" => 3200,
        "f" => 3000,
        "a" => 2500,
        "e" => 2000,
        "w" => 2500,
        _=>{println!("Invalid foo type.");
            return;}
    };

    let mut order:f32 = (price * quantity) as f32;

    if order > 1000.0{
        let discount:f32 = order - (0.05*order);
        println!("You are valid for a 5% discount! New price: {}", discount);
    }else {
       println!("Your price will be: {}",order); 
    }

    






}
