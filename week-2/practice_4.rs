fn main() {
	let p:f64 = 1000.0;
	let r:f64 = 1.0;
	let t:f64 = 2.0;


	/* this is simple interest formula,
	   the one in the example is the compund interest formula
   	*/
	let si = (p * r * t) / 100.0;
	println!("Simple interest is {}", si);

}