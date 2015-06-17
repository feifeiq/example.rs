#![feature(lang_items)]

fn main() {
	let x=1234567u32;
	let mut primes:Vec<u32>=Vec::new();

	let f1=|x:u32|{
		let mut ret=false;
		let mut y=(x as f32).sqrt().floor() as u32;
		while x>=2&&y>1&&!ret{
			ret |= x % y==0;
			y-=1;
		}
		!ret
	};

	for x in 1..1_000_000{
		if f1(x) {print!("{:?},", x);};
	}
	
	println!("{:?}", primes);
	// let f2=|x:u32|{
	// 	match x{
	// 		2=>true,
	// 		n=>n>2&&f1(x,x-1),
	// 	}
	// };

}