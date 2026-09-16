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

    //checks if user is experienced
    if experience == "Yes" {
        is_experienced = true;
    } else {
        is_experienced = false;
    }

    //Asks input for user's age
    println!("What is your age? ");
    io::stdin().read_line(&mut input2).expect("Enter a valid string!");
    let age:u8 = input2.trim().parse().expect("Enter a positive number!");


    if age >= 40 && is_experienced {
        println!("Your annual incentive is: N{:?}", annuals[0]); // Greater than 40 and experienced
    } else if age >= 30 && age <=39 && is_experienced {
        println!("Your annual incentive is: N{:?}", annuals[1]); //Greater than 30 or less than 39 and experienced
    } else if age <= 29 && is_experienced{ 
        /*
            I'm assuming the slide meant 29 and not 28.
            If not, users with age 29 would get an annual incentive of 100k even when experinced
        */
        println!("Your annual incentive is: N{:?}", annuals[2]); //
    } else {
        println!("Your annual incentive is: N{:?}", annuals[3]);
    }

}