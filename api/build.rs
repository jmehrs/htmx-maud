
use std::process::Command;

fn main() {
	let tailwind_cmd = "npx tailwindcss -i input.css -o assets/app.css";

    Command::new("sh")
        .arg("-c")
        .arg(tailwind_cmd)
        .status()
	    .expect("error running tailwind");

}