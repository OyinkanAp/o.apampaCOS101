fn main() {
    let mut v1 = Vec::new();

    let mut input1 = String::new();
    println!("State your FULL occupation(in lowercase): ");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let job:String = input1.trim().parse().expect("Invalid");

    let mut input2 = String::new();
    println!("How many years of work experience do you have? (in format 'x-y years'): ");
    std::io::stdin().read_line(&mut input2).expect("Failed to read input");
    let e:String = input2.trim().parse().expect("Invalid");

    v1.push(job);
    v1.push(e);

    if v1[1] == "1-2 years" && v1[0]== "paralegal lawyer" || v1[0] =="intern" || v1[0] == "placement teacher" {
        println!("This staff holds the position APS 1-2");
    }
    else if v1[1] == "3-5 years" && v1[0] == "administrator" || v1[0] == "academic research assistant" || v1[0] == "junior associate lawyer" || v1[0] == "classroom teacher" {
        println!("This staff holds the position APS 3-5");
    }
    else if v1[1] =="5-8 years" && v1[0] == "senior administrator" || v1[0] == "academic phd candidate" || v1[0] == "associate lawyer" || v1[0] == "snr teacher"  {
        println!("This staff holds the position APS 5-8");
    }
    else if  v1[1] == "8-10 years" && v1[0] == "office manager" || v1[0] == " academic post-doc researcher" || v1[0] == "senior associate 1-2 lawyer" || v1[0] == "leading teacher" {
        println!("This staff holds the position EL1 8-10");
    }
    else if  v1[1] == "10-13 years" && v1[0] == "director" || v1[0] == "academic senior lecturer" || v1[0] == "senior associate 3-4 lawyer" || v1[0] == "deputy principal" {
        println!("This staff holds the position EL2 10-13");
    }
    else if v1[0] == "ceo" || v1[0] == "academic dean" || v1[0] == "partner lawyer" || v1[0] == "principal"{
        println!("This staff holds the position SES");
    }
    else{
        println!("Please rephrase. Invalid sentence.");
        }
}
