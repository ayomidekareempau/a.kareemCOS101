use std::io::*;
use std::io;
use std::fs;

fn main() {
    let mut admin = String::new();
    let mut project = String::new();
    let mut staff = String::new();
    let mut customer = String::new();
    let mut data_plan = String::new();

    let mut input_1 = String::new();

    let mut admin_file = fs::File::open("globacom_dbase.sql").unwrap();
    let mut project_file = fs::File::open("project.sql").unwrap();
    let mut staff_file = fs::File::open("staff.sql").unwrap();
    let mut customer_file = fs::File::open("customer.sql").unwrap();
    let mut data_plan_file = fs::File::open("data_plan.sql").unwrap();

    admin_file.read_to_string(&mut admin).unwrap();
    project_file.read_to_string(&mut project).unwrap();
    staff_file.read_to_string(&mut staff).unwrap();
    customer_file.read_to_string(&mut customer).unwrap();
    data_plan_file.read_to_string(&mut data_plan).unwrap();


    println!("================================================");
    println!("Welcome to Globacom Industries");
    println!("================================================");

    println!("What is your role?
        1. Administrator
        2. Project Manager
        3. Employee
        4. Customer
        5. Vendor"
    );

    io::stdin().read_line(&mut input_1).expect("Enter a valid string!");
    let choice:u8 = input_1.trim().parse().expect("Enter a valid number!");

    match choice {
        1 => {
            println!("Full Database Structure: ");
            println!("{}", admin );
        }
        2 => {
            println!("Project Table: ");
            println!("{}", project );
        }
        3 => {
            println!("Staff Table: ");
            println!("{}", staff );
        }
        4 => {
            println!("Customer Table: ");
            println!("{}", customer );
        }
        5 => {
            println!("Data Plan Table: ");
            println!("{}", data_plan );
        }
        _ => {
            println!("Invalid role!");
        }
    }

}