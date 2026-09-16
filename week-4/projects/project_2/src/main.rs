use std::io;

fn main() {
    let mut input1:String = String::new();
    let mut input2:String = String::new();
    let is_experienced:bool;
    let annuals:[i32; 4] = [
        1_560_000,
        1_480_000,
        1_300_000,
        100_000
    ];

    println!("Are you experienced? (Yes or No)");
    io::stdin().read_line(&mut input1).expect("Enter a valid string!");
    let experience = input1.trim();

    if experience == "Yes" {
        is_experienced = true;
    } else {
        is_experienced = false;
    }

    println!("What is your age? ");
    io::stdin().read_line(&mut input2).expect("Enter a valid string!");
    let age:u8 = input2.trim().parse().expect("Enter a positive number!");

    if age >= 40 && is_experienced {
        println!("Your annual incentive is: N{:?}", annuals[0]);
    } else if age >= 30 && age <=39 && is_experienced {
        println!("Your annual incentive is: N{:?}", annuals[1]);
    } else if age <= 29 && is_experienced{
        println!("Your annual incentive is: N{:?}", annuals[2]);
    } else {
        println!("Your annual incentive is: N{:?}", annuals[3]);
    }

}