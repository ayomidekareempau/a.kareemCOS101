use std::io;

fn main() {
    println!("=========================================");
    println!("RESTUARANT MENU");
    println!("=========================================");

    let mut input_1 = String::new(); //Number of orders
    let mut orders: Vec<(String, f32)> = Vec::new(); //addition of order prices
    let mut total:f32 = 0.0;

    let menu:[(String, f32); 5] = [
        ("Poundo Yam/ Edinako Soup".to_string(), 3200.0),
        ("Fried Rice & Chicken".to_string(), 3000.0),
        ("Amala & Ewedu Soup".to_string(), 2500.0),
        ("Eba & Egusi Soup".to_string(), 2000.0),
        ("White Rice & Stew".to_string(), 2500.0)
    ];

    for (name, price) in &menu {
        println!("\n{} | {} | {}", name.chars().nth(0).unwrap(), name, price ); //Displays menu items neatly
    }

    println!("How much are you ordering");
    io::stdin().read_line(&mut input_1).expect("Expected a string!");
    let order_num:u8 = input_1.trim().parse().expect("Expected a positive number!"); 

    for _ in 1..=order_num{
        let mut input_2 = String::new();
        println!("What are your orders? (P, F, A, E, W)");
        io::stdin().read_line(&mut input_2).expect("Expected a string!");
        let order:char= input_2.trim().parse().expect("Expected a single character");

        match order {
            'P'=> orders.push(menu[0].clone()),
            'F'=> orders.push(menu[1].clone()),
            'A'=> orders.push(menu[2].clone()),
            'E'=>orders.push(menu[3].clone()),
            'W'=>orders.push(menu[4].clone()),
            _ => println!("Enter a valid order")
        }
    }

    println!("Your oders are:");
    for i in &orders{
        println!("{}", i.0);
    }

    for i in &orders{
        total += i.1;
    }

    println!("your total is N{}", total );
    if total >= 10000.0{
        println!("Dicount added, your new total is: N{}", total * 0.95 );
    } 

}