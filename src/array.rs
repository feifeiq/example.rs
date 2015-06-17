use std::mem;
fn analyze_slice(slice:&[i32]){
	println!("1st:{:?}", slice[0]);
	println!("len:{:?}",slice.len() );
}
fn main() {
	let xs:[i32;5]=[1,2,3,4,5];
	let ys:[i32;500]=[0;500];

	println!("first element of the array: {}", xs[0]);
	println!("second element of the array: {}", xs[1]);
	println!("array size: {}", xs.len());
	println!("bytes len:{:?}", mem::size_of_val(&xs));
	analyze_slice(&xs);
	analyze_slice(&ys[1..4]);
	println!("{}", xs[5]);
}