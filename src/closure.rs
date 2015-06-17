static A: i32 = 2;
fn foo(a:&mut i32) -> &i32 {
	*a+=1;
    a
}

fn main() {
	(||{println!("{:?}", "closure test");})();
	let g=|x:i32|{x+1};
	let x=123;
	let foo=|a:&i32|{
		&x
	};

	let (a,b)=(1,2);
	let (a,b)=(b,a);
	println!("{:?}", (a,b));
	
}