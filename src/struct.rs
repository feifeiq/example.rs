#![feature(box_syntax)]
#![feature(core)] 
use std::any::Any;
use std::ops::Deref;
use std::boxed::Box;
use std::fmt::Debug;
use std::marker::Reflect;

struct Nil;
#[derive(Debug)]
struct Pair<'a>(i32,&'a str);

struct Person {
	name: String
}
#[derive(Debug)]
struct Point {
	x: f64,
	y: f64,
}

impl Point {
	fn new() ->Point{
		Point{x:0.3,y:0.4}
	}
}

impl std::default::Default for Point {
	fn default() ->Point{
		Point{x:0.3,y:0.4}
	}
}


struct Node {
    Conns:(u32,Box<Node>),
}
mod m1{
	#[derive(Debug)]
	pub struct WiteBox<T> {
		pub content:T,
	}

	#[derive(Debug)]
	pub struct BlackBox<T> {
		content:T,
	}

	impl<T> BlackBox<T>{
		pub fn new(content:T)->BlackBox<T>{
			BlackBox{content:content}
		}
	}
}
fn main() {

	let white_box=m1::WiteBox{content:"wite box"};
	println!("{:?}",white_box );

	let black_box=m1::BlackBox::new("black box");
	println!("{:?}",black_box );

	println!("{:?}", bar());
	let mut point=Point::new();
	println!("{:?}", point);

	let point:Point=Point{x:0.3,y:0.4};
	println!("({},{})", point.x,point.y);

	let nil=Nil;
	let pair=Pair(1,"a");
	println!("{:?}",pair );
}
