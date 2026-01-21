use std::io::Read;
use std::io;

fn main() {
    let mut input = String::new();
    println!("Please enter your postion in the company(Administrator, Project manager, Employee, Customer or Vendor)(All lowercase.):  ");
    io::stdin().read_line(&mut input).expect("Not a valid response");
    let rs:String = input.trim().parse().expect("Invalid input");

    if rs == "administrator"{
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
    }
    else if rs =="project manager"{
    let mut file = std::fs::File::open("project_table.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
    }
    else if rs == "employee"{
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
    }
    else if rs == "customer"{
    let mut file = std::fs::File::open("customer_table.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
    }
    else if rs == "vendor"{
    let mut file = std::fs::File::open("dataplan_table.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
    }else{println!("Invalid input.")}
}
