use std::io;

fn area_trapezium(height:f64, a:f64, b: f64){
    let area:f64  = (height / 2.0 ) * (a + b);
    println!("The area of the trapezium is: {} ", area);
}

fn area_rhombus(a:f64, b:f64){
    let area:f64 = 0.5 * a * b;
    println!("The area of the rhombus is: {}", area);
}

fn area_parallelogram(base: f64, altitude: f64){
    let area:f64 = base * altitude;
    println!("The area of the parallelogram is: {}", area );
}

fn surface_area_cube(side: f64){
    let s_area = 6.0 * side * side;
    println!("The surface area of the cube: {}", s_area );
}

fn volume_cylinder(radius: f64, height:f64){
    let pi:f64 = 22.0 / 7.0;
    let volume:f64 = pi * radius.powf(2.0) * height;

    println!("The volume of the cylinder is: {}", volume );
}

fn main() {
    println!("=============================================");
    println!("THE SHAPE CALCULATOR");
    println!("=============================================");

    let mut calculate_again:bool = true;

    while calculate_again {
        let mut input1 = String::new();
        let mut input2 = String::new();
        let mut input3 = String::new();
        let mut input4 = String::new();
        let mut input5 = String::new(); //try again logic

        println!("
            1. Calculate area of trapezium \n
            2. Calculate area of rhombus\n
            3. Calculate area of parallelogram \n
            4. Calculate surface area of cube \n
            5. Calculate volume of cylinder \n
        ");

        println!("Choose an option to calculate");
        io::stdin().read_line(&mut input1).expect("Expected valid input");
        let choice:u8 = input1.trim().parse().expect("Expected a valid number");

        match choice {
            1 => {
                println!("Enter first base: ");
                io::stdin().read_line(&mut input2).expect("Expected valid input");
                let a:f64 = input2.trim().parse().expect("Expected a valid number");

                println!("Enter second base: ");
                io::stdin().read_line(&mut input3).expect("Expected valid input");
                let b:f64 = input3.trim().parse().expect("Expected a valid number");

                println!("Enter height");
                io::stdin().read_line(&mut input4).expect("Expeced a valid input");
                let c:f64 = input4.trim().parse().expect("Expected a valid number");

                area_trapezium(c, a, b);
            },
            2 => {
                println!("Enter first diagonal: ");
                io::stdin().read_line(&mut input2).expect("Expected a valid input");
                let d1:f64 = input2.trim().parse().expect("Expected a valid number");

                println!("Enter second diagonal: ");
                io::stdin().read_line(&mut input3).expect("Expected a valid input");
                let d2:f64 = input3.trim().parse().expect("Expected a valid number");

                area_rhombus(d1, d2);
            },
            3 => {
                println!("Enter the base: ");
                io::stdin().read_line(&mut input2).expect("Expected a valid input");
                let base:f64 = input2.trim().parse().expect("Expected a valid number");

                println!("Enter the altidude: ");
                io::stdin().read_line(&mut input3).expect("Expected a valid input");
                let altitude:f64 = input3.trim().parse().expect("Expected a valid number");

                area_parallelogram(base, altitude);
            },
            4 => {
                println!("Enter the length of side: ");
                io::stdin().read_line(&mut input2).expect("Expected a valid input");
                let side:f64 = input2.trim().parse().expect("Expected a valid number");

                surface_area_cube(side);
            },
            5 => {
                println!("Enter the radius: ");
                io::stdin().read_line(&mut input2).expect("Expected a valid input");
                let radius:f64 = input2.trim().parse().expect("Expected a valid number");

                println!("Enter the height: ");
                io::stdin().read_line(&mut input3).expect("Expected a valid input");
                let height:f64 = input3.trim().parse().expect("Expected a valid number");

                volume_cylinder(radius, height);
            }
            _ => println!("Invalid input"),
        }

        println!("Would u like to calculate again");
        io::stdin().read_line(&mut input5).expect("Expected a valid input");
        let calculate:String = input5.trim().parse().expect("Expected a valid input");

        if calculate.to_lowercase() == "yes" {
            calculate_again = true;
        } else {
            calculate_again = false;
            println!("Byee!");
        }
    }
}