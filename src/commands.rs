use std::process::Command;

pub fn test_command(arg: &String) {
	let mut cmd = Command::new("code");
	cmd.arg(arg);
	let output = cmd.output().expect("could not run command!");
	
	println!("status: {}", output.status);
	println!("output: {}", String::from_utf8_lossy(&output.stdout));
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum CommandType {
	#[default] ToMp3,
	ToWav,
	ResizeVideo,
	CropVideo
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Quality {
	#[default] High,
	Medium,
	Low
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ToMp3Arguments {
	pub constant_bitrate: bool,
	pub quality: Quality,
}

pub fn to_mp3(input_file_path: &String, args: ToMp3Arguments) {
	let mut cmd = Command::new("ffmpeg");
	
	// trim and split the string, iterate all segments just in case we get a source path with a bad.naming.convention somewhere.
	let trimmed_path = input_file_path.trim();
	let split_path = trimmed_path.split('.');
	let e = split_path.enumerate();
	let size = &e.clone().count();
	let mut output_path = String::from("");
	let mut i = 0;
	for segment in e {
		if i < size - 1 {
			output_path = output_path.to_owned() + segment.1 + ".";
		}
		i = i+1;
	}
	output_path = output_path.to_owned() + "mp3";

	let bitrate_flag = if args.constant_bitrate { "-b:a" } else { "-q:a" };
	let mut quality = "";
	if args.constant_bitrate {
		quality = match args.quality {
			Quality::High => if args.constant_bitrate { "320k" } else { "0" },
			Quality::Medium => "4",
			Quality::Low => "8", 
		}
	}

	cmd.args(["-i", input_file_path, bitrate_flag, quality, &output_path]);
	let output = cmd.output().expect("could not run command!");

	println!("status: {}", output.status);
	println!("output: {}", String::from_utf8_lossy(&output.stdout));
	println!("error: {}", String::from_utf8_lossy(&output.stderr));
}
