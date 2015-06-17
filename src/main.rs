#![warn(unused_imports)]
use std::io;
use std::mem;

fn main(){
	

	let mut buf_mem = [0u8;20];
	for i in 0..20 {
		buf_mem[i]=i as u8;
	}

	let mut bufs=[&buf_mem[..1];10];
	if let Some(x)=Some(bufs[0]){
		println!("{:?}", x==[]);
	}
	println!("{:?}", bufs);
	for i in 0..10 {
		let buf=&buf_mem[i*2..i*2+1];
		bufs[i] = buf;
	}
	println!("{:?}", bufs);

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