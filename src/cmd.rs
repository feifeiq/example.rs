use std::process::Command;

use std::ffi::OsStr;


fn main() {

	let output = Command::new("ipconfig").arg("/all").output().unwrap_or_else(|e| {
		panic!("failed to execute process: {}", e)
	});

	println!("status: {}", output.status);
	println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
	println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
