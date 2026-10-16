use std::io;

fn main() {
    let mut input_1 = String::new();

    let mut people:Vec<(String, i32)> = Vec::new();

    println!("How many users?");
    io::stdin().read_line(&mut input_1).expect("Expected a valid string!");
    let users:u8 = input_1.trim().parse().expect("Expected a valid number!");

	for _ in 1..=users{
	    let mut input_2 = String::new();
		let mut input_3 = String::new();

    	println!("Enter your name");
    	io::stdin().read_line(&mut input_2).expect("Expected a valid string!");
    	let name = input_2.trim();

    	println!("Enter your years of experience");
    	io::stdin().read_line(&mut input_3).expect("Expectewd a valid string!");
    	let years:i32 = input_3.trim().parse().expect("Expectd a valid number!");
    	people.push((name.to_string(), years));
	}	

	let highest = people.iter().max();
	println!("The Person with the highest experience: ");
	println!("Name: {},  Year: {}", highest.unwrap().0, highest.unwrap().1 );

}