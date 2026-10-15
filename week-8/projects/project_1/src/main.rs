use std::io;

fn main() {
    let a = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let b = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let c = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let d = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

    let mut input_1 = String::new();
    let mut input_2 = String::new();

    println!("What category do u fall under?
        1. Office Admin
        2. Academic
        3. Lawyer
        4. Teacher");

    io::stdin().read_line(&mut input_1).expect("Enter valid input!");
    let category:u8 = input_1.trim().parse().expect("Enter valid input");
    let mut count:u8 = 0;

    match category {
        1 => {
            println!("What is your role? ");  
            for i in a {
                count+=1;
                println!("{}. {}", count, i);
            }
            io::stdin().read_line(&mut input_2).expect("Expected valid input");
            let role:u8 = input_2.trim().parse().expect("Expected a valid number!");
            calculate_aps(role);
        }
        2 => {
            println!("What is your role? ");  
            for i in b {
                count+=1;
                println!("{}. {}", count, i);
            }
            io::stdin().read_line(&mut input_2).expect("Expected valid input");
            let role:u8 = input_2.trim().parse().expect("Expected a valid number!");
            calculate_aps(role);
        }
        3 => {
            println!("What is your role? ");  
            for i in c {
                count+=1;
                println!("{}. {}", count, i);
            }
            io::stdin().read_line(&mut input_2).expect("Expected valid input");
            let role:u8 = input_2.trim().parse().expect("Expected a valid number!");
            calculate_aps(role);
        }
        4 => {
            println!("What is your role? ");  
            for i in d {
                count+=1;
                println!("{}. {}", count, i);
            }
            io::stdin().read_line(&mut input_2).expect("Expected valid input");
            let role:u8 = input_2.trim().parse().expect("Expected a valid number!");
            calculate_aps(role);
        }
        _ => {
            println!("Enter valid input");
        }
    }
}

fn calculate_aps(role: u8){
    let aps: [&str; 6] = ["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];
    if role == 1 {
        println!("Your Aps Level is:  {}" , aps[0] );
    } else if role == 2 {
        println!("Your Aps Level is:  {}" , aps[1] );
    } else if role == 3 {
        println!("Your Aps Level is:  {}" , aps[2] );
    } else if role == 4 {
        println!("Your Aps Level is:  {}" , aps[3] );
    } else if role == 5 {   
        println!("Your Aps Level is:  {}" , aps[4] );
    } else if role == 6 {
        println!("Your Aps Level is:  {}" , aps[5] );
    } else {
        println!("Cannot calulate aps!");
    }
}