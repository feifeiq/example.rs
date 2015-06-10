struct UnPrintable(i32);
#[derive(Debug)]
struct DebugPrintable(i32);

fn main() {
	println!("{:?}", DebugPrintable(123));
}