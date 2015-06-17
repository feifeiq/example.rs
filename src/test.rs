fn main() {
	let mut count = 0;
	println!("{:?}",count); 
	{
		let mut inc= || {
			count += 1;
			println!("`count`: {}", count); 
		};
		inc();
		inc();
	}
	// println!("{}",count);
	count += 1;
}