#![feature(zero_one)]
extern crate num;
use std::ops::Add;
use std::num::One;
use num::Num;
// trait Incr {
//     fn inc(&mut self);
// }

// impl<T: Add> Incr for T {
//     fn inc(&mut self) { *self = *self + 1; }
// }

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
}