#![feature(box_syntax)]
#![feature(core)] 
use std::any::Any;
use std::ops::Deref;
use std::boxed::Box;
use std::fmt::Debug;
use std::marker::Reflect;
struct Person {
	name: String
}
#[derive(Debug)]
struct Point {
	x: i32,
	y: i32,
}

impl Point {
	fn new() ->Point{
		Point{
			x:1,
			y:2,
		}
	}
}

impl std::default::Default for Point {
	fn default() ->Point{
		Point{
			x:1,
			y:2,
		}
	}
}
fn foo<T:Debug+Reflect+'static>(x:Box<Any>){
	println!("{:?}", x.downcast::<T>().ok().unwrap());
}

fn main() {
	let mut x=123u8;
	foo::<u8>(box x);
	let mut y=&x;
	foo::<&u8>(box y);
	foo::<i32>(box 123);
	foo::<&str>(box "abc");
	foo::<String>(box "abc".to_string());

	let x:Box<Any>=box 1;
	println!("{:?}", x.downcast::<i32>().ok().unwrap());

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
	let mut point=Point::new();
	println!("{:?}", point);
}
