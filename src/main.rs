#![warn(unused_imports)]
use std::io;
use std::mem;

fn main(){
	let  x=6;
	let ret=match x {
		5 => "E",
		6 => "D",
		7 => "C",
		8...9=>"B",
		10=>"A",
		_=>"?"
	};
	println!("{:?}", ret);

	let x=vec![1,2,3];
	let mut x=x;
	x.push(5);
	println!("{:?}",x);

	let x=5;
	let y=&x;
	println!("{:?}", *y);
	println!("{:?}", y);

	
	unsafe {
		let a = [0u8, 0u8, 0u8, 0u8];

		let a = mem::transmute::<[u8; 4], u32>(a);
	}
	let (x,y)=(5,6);
	let i:i32=123;
	println!("x={}", x);
	println!("{a}", a=12);

	let mut num = 5;

	{ 
		let mut add_num = |x: i32| num += x;

		add_num(5);
	}

	assert_eq!(10, num);
}