use std::io;

fn main() {
    println!("=========================================");
    println!("RESTUARANT MENU");
    println!("=========================================");

    let mut input_1 = String::new(); //Number of orders
    let mut orders: Vec<(String, f32)> = Vec::new();
    let mut total:f32 = 0.0;

    let menu:Vec<(String, f32)> = vec![
            ("Poundo Yam/ Edinako Soup".to_string(), 3200.0),
            ("Fried Rice & Chicken".to_string(), 3000.0),
            ("Amala & Ewedu Soup".to_string(), 2500.0),
            ("Eba & Egusi Soup".to_string(), 2000.0),
            ("White Rice & Stew".to_string(), 2500.0)
        ];

    for (name, price) in &menu {
        println!("\n{} | {} | {}", name.chars().nth(0).unwrap(), name, price );
    }

    println!("How much are you ordering");
    io::stdin().read_line(&mut input_1).expect("Expected a string!");
    let order_num:u8 = input_1.trim().parse().expect("Expected a positive number!");

    for _ in 1..=order_num{
        let mut input_2 = String::new();
        println!("What are your orders? (P , F, A, E, W)");
        io::stdin().read_line(&mut input_2).expect("Expected a string!");
        let order:char= input_2.trim().parse().expect("Expected a single character");

        if order == 'P'{
            orders.push(menu[0].clone())
        } 
        else if order == 'F' {
            orders.push(menu[1].clone())
        }
        else if order == 'A' {
            orders.push(menu[2].clone())
        }
        else if order == 'E' {
            orders.push(menu[3].clone())
        }
        else if order == 'W' {
            orders.push(menu[4].clone())
        } else{
            println!("Guy what are you ordering?");
        }
    }

    for i in &orders{
        total += i.1;
    }

    if total >= 10000.0{
        println!("Your total is {}", total * 0.95 );
    } else if total == 0.0 {
        println!("Enter valid input");
    } 
    else {
        println!("your total is N{}", total );
    }
}