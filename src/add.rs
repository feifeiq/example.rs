#![feature(zero_one)]
#![feature(box_syntax)]

extern crate num;
use std::ops::Add;
use std::num::One;
use num::Num;
use std::any::Any;
use std::fmt::Debug;
use std::marker::{Reflect,Sized};
use std::ops::Add;
// trait Incr {
//     fn inc(&mut self);
// }

// impl<T: Add> Incr for T {
//     fn inc(&mut self) { *self = *self + 1; }
// }


fn foo<T>(x: Box<Any>) where T:Any+Debug{
	println!("{:?}", x.downcast::<T>().unwrap());
}


fn inc<T>(x:&mut T) where T:Add<Output=T> + One+Copy{
	*x=*x+One::one();
}

fn succ<T>(x:&mut T) where T:Num+Copy{
	*x=*x+T::from_str_radix("1",10).ok().unwrap();
}

fn main() {

	let mut x=123;
	inc(&mut x);
	succ(&mut x);
	println!("{:?}", x);

	foo::<i32>(box 123);
	foo::<&str>(box "123");
	foo::<String>(box "123".to_string());
	let x=2u8;
	foo::<u8>(box x);
	println!("{:?}", x);;
}