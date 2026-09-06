fn main() {

	let prices:[f32; 5] = [
		450000.0, 
		1500000.0, 
		750000.0, 
		2850000.0,
		250000.0,
	];

	let quantity:[f32; 5] = [
		2.0,
		1.0,
		3.0,
		3.0,
		1.0,
	];

	let total:f32 = 
	(prices[0] * quantity[0]) 
	+ (prices[1] * quantity[1]) 
	+ (prices[2] * quantity[2])
	+ (prices[3] * quantity[3])
	+ (prices[4] * quantity[4]);

	let average:f32 = total / quantity.iter().sum::<f32>();

	println!("The sum of the sales record is: {}", total );
	println!("The average of the sales record is: {}", average );

}
