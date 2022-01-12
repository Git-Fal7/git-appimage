extern crate serde;
extern crate serde_json;
extern crate nix;

use serde::{Serialize, Deserialize};

use nix::unistd::Uid;

use std::fs;
use std::fs::File;
use std::env;

use std::process::Command;

use std::io::{BufRead, BufReader};

use std::path::Path;
use std::io::Write;


#[derive(Serialize, Deserialize, Debug)]
struct About {
	tag_name: String,
	assets: Vec<URL>,
}
#[derive(Serialize, Deserialize, Debug)]
struct URL {
	name: String,
	browser_download_url: String,
}

struct Info {
	file: String,
	tag: String,
	url: String,
}

fn main() {

	if !Uid::effective().is_root(){
		panic!("You should be root");
	}
	
	let args: Vec<String> = env::args().collect();
	
	if args.len() == 1 ||
	 ! (args[1].contains("/")) || 
	 args[1].starts_with("/") || 
	 args[1].ends_with("/") {
		panic!("Please put a github repo");
	}
	
	let out = Command::new("curl")
	 .arg("-H")
	 .arg("Accept: application/vnd.github.v3+json")
	 .arg(format!("https://api.github.com/repos/{}/releases", &args[1]))
	 .output()
	 .expect("failed to execute");
	let put = String::from_utf8_lossy(&out.stdout);
	let er = format!("{}", put);
	if er.contains("documentation_url") && 
	  er.contains("https://docs.github.com/rest/reference/repos#list-releases") {
		panic!("Invalid github repo");
	}
	let mut aimages: Vec<Info> = Vec::new();
	
	let json: Vec<About> = serde_json::from_str(&er).unwrap();
	
	for i in 0..json.len(){
		for x in 0..json[i].assets.len() {
			if json[i].assets[x].name.ends_with(".AppImage") {
				let aimg = Info {
					file: format!("{}", json[i].assets[x].name),
					tag: format!("{}", json[i].tag_name),
					url: format!("{}", json[i].assets[x].browser_download_url),
				};
				aimages.push(aimg);
			}
		}
	}
	if aimages.is_empty() {
		panic!("Theres no release with an appimage");
	}
	for i in 0..aimages.len() {
		println!("");
		println!("{}", i);
		println!("{}", aimages[i].file);
		println!("{}", aimages[i].tag);
		println!("{}", aimages[i].url);
	}

	if args.len() > 2 {
		let choose = &args[2];
		let arint: u16 = choose.parse().unwrap();
		let me = arint as usize;
		if me < aimages.len() {
			println!("{}", &aimages[me].url);
			Command::new("wget")
			.arg(&aimages[me].url)
			.output()
			.expect("failed to run cmd");
			let fname = &aimages[me].file;
			let filen = Path::new(fname);
			if !(filen.exists()) {
				panic!("File Doesn't exist");
			} else {
				println!("Fi;le exists");
			}
			Command::new("chmod")
			 .arg("+x")
			 .arg(filen)
			 .output()
			 .expect("Failed to execute");
			println!("chmoding appimage");
			fs::create_dir_all("/opt/appimages").ok();
			fs::copy(filen, format!("/opt/appimages/{}", filen.display())).ok();
			Command::new(format!("./{}", filen.display()))
			 .arg("--appimage-extract")
			 .output()
			 .expect("Failed to execute");
			println!("extracted");
			
			let sqfs = fs::read_dir("squashfs-root/").unwrap();
			
			for files in sqfs {
				let fi = format!("{}", files.unwrap().path().display());
				if fi.to_lowercase().ends_with(".desktop") {
					let replacewith = format!("Exec=/opt/appimages/{}", fname);
					let f = File::open(fi).expect("failed to open file");
					let f = BufReader::new(f);
					let path = format!("/usr/share/applications/{}.desktop", fname);
					let mut output = File::create(path).expect("failed to create file");
					for line in f.lines() {
						let mut line = line.expect("failed to read line");
						if line.starts_with("Exec=") {
							let replace: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
							line = line.replace(&replace[0], &replacewith);
						}
						writeln!(output, "{}", line).expect("Failed to write");
					}
				}
				else if fi.to_lowercase().ends_with(".png"){
					let filename: Vec<String> = fi.split("/").map(|s| s.to_string()).collect();
					fs::copy(fi, format!("/usr/share/pixmaps/{}", &filename[1])).ok();
				}
				else if fi.to_lowercase().ends_with(".svg") {
					let filename: Vec<String> = fi.split("/").map(|s| s.to_string()).collect();
					fs::copy(fi, format!("/usr/share/pixmaps/{}", &filename[1])).ok();
				}
			}

			fs::remove_dir_all("squashfs-root/").ok();
			println!("sucessfully installed");
		}
	}
}
