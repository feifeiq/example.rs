#![feature(slice_patterns)]
fn main() {
	let n=6;
	match n {

		2|4=>println!("{:?}", "2|4"),
		6|8=>println!("{:?}", "6|8"),
		x if x%2==0 =>println!("{:?}", "偶数"),
		e@ 1 |e@3|e@5=>println!("{:?}", e),
		e@ 1...9=>println!("range {:?}",e ),
		_=>println!("{:?}", "no match"),
	};

	let  x=6;
	let ret=match x {
		5 => "E",
		6 => "D",
		7 => "C",
		8...9=>"B",
		10=>"A",
		_=>"?"
	};
	println!("{:?}", ret);

}