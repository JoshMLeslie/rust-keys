// build.rs
fn main() {
	#[cfg(target_os = "linux")]
	{
			let output = std::process::Command::new("pkg-config")
					.args(["--exists", "alsa-sys"])
					.status()
					.expect("Failed to run pkg-config");

			if !output.success() {
					panic!("Missing libasound2-dev. Please install: sudo apt install libasound2-dev");
			}
	}
}
