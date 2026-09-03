fn main() {
	let toshiba_price:f32 = 450000.0;
	let mac_price:f32 = 1500000.0;
	let hp_price:f32 = 750000.0;
	let dell_price:f32 = 2850000.0;
	let acer_price:f32 = 250000.0;

	let toshiba_qty:f32 = 2.0;
	let mac_qty:f32 = 1.0;
	let hp_qty:f32 = 3.0;
	let dell_qty:f32 = 3.0;
	let acer_qty:f32 = 1.0;

	let sum:f32 = (toshiba_price * toshiba_qty) 
	+ (mac_price * mac_qty) + (hp_price * hp_qty)
	 + (dell_price * dell_qty) + (acer_price * acer_qty);

	let average:f32 = sum / (toshiba_qty + mac_qty + hp_qty + dell_qty + acer_qty); 

	println!("The sum of the sales record is: {}", sum );
	println!("The average of the sales record is: {}", average );

}
