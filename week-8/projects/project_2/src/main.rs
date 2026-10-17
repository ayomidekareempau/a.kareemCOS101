use std::io;
use std::io::Write;

fn main() {
    let mut input_1 = String::new();
    let mut people:Vec<(String, i32)> = Vec::new(); //name, yrs_exp

    println!("==================================================");
    println!("Ernst & Young (EY) Global Limited Scouter Program!");
    println!("==================================================");

    print!("Enter number of users: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_1).expect("Expected a valid string!");
    let users:u8 = input_1.trim().parse().expect("Expected a valid number!");

	for _ in 1..=users{ 
	    let mut input_2 = String::new(); //name
		let mut input_3 = String::new(); //years of exp

    	print!("Enter name: ");
    	io::stdout().flush().unwrap();
    	io::stdin().read_line(&mut input_2).expect("Expected a valid string!");
    	let name = input_2.trim();

    	print!("Enter your years of experience: ");
    	io::stdout().flush().unwrap();
    	io::stdin().read_line(&mut input_3).expect("Expectewd a valid string!");
    	let years:i32 = input_3.trim().parse().expect("Expectd a valid number!");

    	//stores each user and their corresponding yrs of exp expressed as a tuple in the vector 
    	people.push((name.to_string(), years)); 
	}	

	let mut max_user_name = &people[0].0; //first person's name
	let mut max_user_years = people[0].1; //first person's yrs_exp

	if people.is_empty(){
		println!("No user has been inputed!");
	}else {
		/*This for loop iterates though all the tuple values(name, yrs_exp) in the vector
		  then checks if the tuples 2nd index(yrs_exp) is greater than the first one, if it is..
		  it stores the higher value in max_user_years then checks again and only changes when
	   	  it finds a higher one. The corresponding name with the highest yrs_exp value is then stores in the 
	   	  max_user_name 
		  */
		for peeps in &people {
			if peeps.1 > max_user_years {
				max_user_years = peeps.1;
				max_user_name = &peeps.0;
			}
		}
		println!("{} with {} number of years has the highest experience", max_user_name, max_user_years);
	}
}