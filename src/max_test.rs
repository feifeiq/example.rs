use std::fmt::Display;

trait GetMax {
    fn max_value() -> Self;
}

impl GetMax for i32 {
    fn max_value() -> i32 {
        i32::max_value()
    }
}

fn test<T: GetMax + Display>() {
	println!("max value {}", T::max_value());
}


fn main() {
    test::<i32>();
    // test(1i32);
}