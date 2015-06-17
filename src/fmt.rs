use std::fmt::{Display,Formatter,Result};

struct City {
	name: &'static str,
	lat:f32,
	lon:f32,
}

impl Display for City {
	fn fmt(&self,f:&mut Formatter)->Result{
		let lat= if self.lat>0.0 {"N"} else {"S"};
		let lon=if self.lon>0.0{"E"} else{"W"};
		write!(f,"{}:{}{:.3},{}{:.3}",self.name,lat,self.lat,lon,self.lon)
	}
}

fn main() {

	let foo=1234567;
	println!("0x{:x}", foo);
	println!("0{:o}", foo);
	println!("0b{:b}", foo);
	println!("{:p}", &foo);
	
	let cities=[City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
	City { name: "Oslo", lat: 59.95, lon: 10.75 },
	City { name: "Vancouver", lat: 49.25, lon: -123.1 },];

	for city in cities.iter()  {
		println!("{}", city);
	}
}