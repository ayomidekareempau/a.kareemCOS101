fn main() {
	let p:f32 = 210000.0;
	let r:f32 = 5.0;
	let n:f32 = 3.0;

	let a:f32 = p * (1.0 - (r / 100.0 )).powf(n);
	println!("The value of the TV has deprecated to: {}", a);

}

