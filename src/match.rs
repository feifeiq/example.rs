fn main() {

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

	let b = Some(box 5);
	match b {
		Some(box n) if n < 0 => {
			println!("Box contains negative number {}", n);
		},
		Some(box n) if n >= 0 => {
			println!("Box contains non-negative number {}", n);
		},
		None => {
			println!("No box");
		},
		_ => unreachable!()
	}
}