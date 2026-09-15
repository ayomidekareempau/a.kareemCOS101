use std::*;

fn main() {

    let mut try_again = true;

    while try_again {

        let mut a_input= String::new();
        let mut b_input = String::new();
        let mut c_input = String::new();
        let mut again = String::new(); //Option to try again or not

        //Value for a
        println!("Value of a? ");
        io::stdin().read_line(&mut a_input).expect("Expected a string");
        let a:f64 = a_input.trim().parse().expect("Please enter a valid number next time");

        //Value for b
        println!("Value of b? ");
        io::stdin().read_line(&mut b_input).expect("Expected a string");
        let b:f64 = b_input.trim().parse().expect("Please enter a valid number next time");

        //Value for c
        println!("Value of c? ");
        io::stdin().read_line(&mut c_input).expect("Expected a string");
        let c:f64 = c_input.trim().parse().expect("Please enter a valid number next time");

        //Discriminant formula
        let d:f64= b.powf(2.0) - 4.0 * a * c;

        let pos_quadratic:f64 = (-b + d.sqrt()) / (2.0 * a); //positive side of the quadratic equation
        let neg_quadratic:f64 = (-b - d.sqrt()) / (2.0 * a); //negative side of the quadratic equation

        if d < 0.0{
            println!("No roots"); //Gives this outputs since you can't squareroot negative numbers
        } else if pos_quadratic == neg_quadratic {
            println!("The quadratic root is {}", pos_quadratic ); 
        }
        else {
            println!("The quadratic roots of the equations are {} and {}", pos_quadratic, neg_quadratic);
        }

        //Added option to calculate another set of valus
        println!("Calculate again?");
        io::stdin().read_line(&mut again).expect("Enter a valid string");

        //will always rerun the program unless the user inputs no
        if again.trim().to_lowercase() == "no" {
            try_again = false;
            println!("Byeee!");
        } else {
            try_again = true;
        }
    }    
}