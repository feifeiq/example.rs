use std::fmt;

struct Printable(i32);

impl fmt::Display for Printable{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,"{}",self.0)
	}
}

struct List(Vec<i32>);

impl fmt::Display for List {
	fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
		let vec:Vec<i32>;
		let &List(ref vec)=self;
		let len=vec.len();

		for (i,v) in vec.iter().enumerate(){
			if i<len-1{
				write!(f,"{},",v);
			}
		}

		write!(f,"{:?}",vec[len-1])
	}
}

fn main() {
	let p=(1,2);
	let (x,y)=p;
	println!("x={},y={}", x,y);
	println!("{}", Printable(123));

	let v=List(vec![1,2,3]);
	println!("{}", v);
}