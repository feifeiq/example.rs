static A: i32 = 2;
fn foo(a:&mut i32) -> &i32 {
	*a+=1;
	a
}

fn main() {

	let fac=|x:u32|{
		let mut ret=1;
		let mut n=x;
		while n>1 {
			ret*=n;
			n-=1;
		}
		println!("fac {:?}", (x,ret));
		ret
	};
	fac(10);
	// let f1=|x:i32|{
	// 	x%2==0
	// };
	// let f2=|x:i32|{
	// 	println!("{:?}", (x,f1(x)));
	// };
	// f2(123);
	// f2(12);

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