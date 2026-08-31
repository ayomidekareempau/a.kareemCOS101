fn main() {
	let p:f32 = 520000000.0;
	let r:f32 = 10.0;
	let n:f32 = 5.0;

	let a = p * (1.0 + (r / 100.0)).powf(n) ;
	let ci = a - p;
	println!("The Compound Interest is {}", ci);
}

