use std::process::Command;
use std::path::PathBuf;

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
	High,
	#[default]Medium,
	Low,
	Broken
}

pub fn to_mp3(input_file_path: &Option<PathBuf>, quality: Quality) {
	let mut cmd = Command::new("ffmpeg");
	// TODO: there should be neat ways of doing all of the below using PathBuf instead of working on a String.
	
	// to string
	let path_string = input_file_path.clone().expect("no valid path").into_os_string().into_string().unwrap();
	// trim and split the string, iterate all segments just in case we get a source path with a bad.naming.convention somewhere.
	let trimmed_path = path_string.trim();
	let split_path = trimmed_path.split('.');
	let e = split_path.enumerate();
	let size = &e.clone().count();
	let mut output_path = String::from("");
	let mut i = 0;
	for segment in e {
		if i < size - 1 {
			output_path = output_path.to_owned() + segment.1;
		}
		i = i+1;
	}

	let mut version:usize = 0;
	loop {
		let out = format!(
			"{}{}",
			output_path,
			if version == 0 {"".to_string()} else {"_".to_owned() + &version.to_string()}
		);
		if std::fs::exists(out.to_owned() + ".mp3").unwrap() {
			version = version + 1;
		} else {
			output_path = out.to_owned() + ".mp3";
			break;
		}
	}

	// output_path = output_path + ".mp3";
	//output_path = String::from("output.mp3");

	// let bitrate_flag = if args.constant_bitrate { "-b:a" } else { "-q:a" };
	let bitrate_flag = "-b:a";
	let quality = match quality {
		Quality::High => "320k",
		Quality::Medium => "128k",
		Quality::Low => "64k",
		Quality::Broken => "32k", 
	};

	cmd.args(["-i", &path_string, bitrate_flag, quality, &output_path]);
	let output = cmd.output().expect("could not run command!");

	println!("status: {}", output.status);
	println!("output: {}", String::from_utf8_lossy(&output.stdout));
	println!("error: {}", String::from_utf8_lossy(&output.stderr));
}
