trait Foo {
	fn method(&self) -> String;
}

impl Foo for u8 {
	fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
	fn method(&self) -> String { format!("string: {}", *self) }
}
// fn do_something<T: Foo>(x: T) {
// 	x.method();
// }

fn do_something(x: &Foo) {
	x.method();
}

	/// let x = 5;
	/// #let y = 6;
	/// println!("{}", x + y);
	fn main() {
		let x = 5u8;
		do_something(&x as &Foo);
		println!("{:?}", x);

		let y={
			let x = "Hello".to_string();
			do_something(&x);
			println!("{:?}", x);
		};
		println!("{:?}", x);
		let x = 5u8;
		let y = "Hello".to_string();
		

	// do_something(x);
	// do_something(y);

}