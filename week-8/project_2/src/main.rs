fn main() {
    let mut names:Vec<String> = Vec::new();
    let mut experience:Vec<i32> = Vec::new();

    let mut in1 = String::new();
    println!("Enter the number of candidates: ");
    std::io::stdin().read_line(&mut in1).expect("Failed to read input");
    let num:i32 = in1.trim().parse().expect("Invalid input");

    for num in 1..=num{
        let mut in2 = String::new();
        println!("\nEnter the name of the candidate: ");
        std::io::stdin().read_line(&mut in2).expect("Failed to read input");
        let name:String = in2.trim().parse().expect("Invalid input");

        let mut in3 = String::new();
        println!("Enter their years of experience: ");
        std::io::stdin().read_line(&mut in3).expect("Failed to read input");
        let exp:i32 = in3.trim().parse().expect("Invalid input");
        names.push(name);
        experience.push(exp);
    }

    let mut best = 0;
    for i in 1..experience.len(){
        if experience[i] > experience[best]{
             best = i;
        }
    }
    println!("The candidate with the highest years of programming experience is {:?}", names[best]);

}
