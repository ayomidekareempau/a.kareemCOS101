struct Laptop {
    name: String,
    price: u32
}

impl Laptop {
    fn total_by_three(&self, quantity:u32) -> u32 {
        return self.price * quantity;
    }
}

fn main() {
    let quantity = 3;
    let hp = Laptop {
        name: "HP".to_string(),
        price: 650000,
    };
    let ibm = Laptop {
        name: "IBM".to_string(),
        price: 755000,
    };
    let toshiba = Laptop {
        name: "Toshiba".to_string(),
        price: 550000,
    };
    let dell = Laptop {
        name: "Dell".to_string(),
        price: 850000,
    };

    let laptops:Vec<Laptop> = vec![hp, ibm, toshiba, dell];

    let mut final_cost = 0;

    for i in laptops{
        let cost = i.total_by_three(quantity);
        println!("The total cost from {} is N{}", i.name, cost );
        final_cost += cost;
    }

    println!("The final cost from each brand is N{}", final_cost );

}