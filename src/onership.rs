fn foo(v1: &mut Vec<i32>, v2: &Vec<i32>) -> i32 {
    // do stuff with v1 and v2
    v1.push(5);
    // return the answer
    42
}
fn main() {
	let mut v1 = vec![1, 2, 3];
	let v2 = vec![1, 2, 3];

	let answer = foo(&mut v1, &v2);

	// we can use v1 and v2 here!
	println!("{:?}", (v1,v2));

	let x: &'static str = "Hello, world.";

	let mut v = vec![1, 2, 3];

	for i in &v {
		println!("{}", i);
		// v.push(34);
	}

	let mut x = 5;
	{                   
   	 	let y = &mut x;     // -+ &mut borrow starts here
   	 	*y += 1;            //  |
   	 	println!("y={}", y);//  |
	}                	    // -+ ... and ends here
	println!("x={}", x);	// <- try to borrow x here
	{
		let y = &mut x;
		*y += 1;
	}
	// let z=&x;
	let y = &mut x;
	*y += 1;

}