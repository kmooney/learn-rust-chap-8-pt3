use std::io::{self, Write};
use std::collections::HashMap;
use regex::Regex;

fn add_to_directory(d: &mut HashMap<String, Vec<String>>, who: String, dept: String) {
	let dept_v: &mut Vec<String> = d.entry(dept).or_insert(Vec::<String>::new());
	dept_v.push(who);
}

fn print_dir(d: &HashMap<String, Vec<String>>) {
	for department in d.keys() {
		println!("department: {}", department);
		println!("members: {:?}", d[department]);
	}
}

fn main() {
    println!("departments v1.0! type: 'help' for help");
    let mut directory: HashMap<String, Vec<String>> = HashMap::new();
    let add_cmd = Regex::new(r"^add (?P<who>[a-zA-Z]+) to (?P<department>[a-zA-Z]+)$").unwrap();
    loop {
    	let mut input_string = String::new();

	    print!("> ");
	    io::stdout().flush().unwrap();
	    let bytes_read = io::stdin().read_line(&mut input_string).unwrap();
	    let input_slice = input_string.trim();
	    if bytes_read == 0 {
	    	eprintln!("EOF? okay bye!");
	    	break;
	    }

	    if input_slice == "help" {
	    	println!("add X to Y: adds person X to department Y.  creates dept. Y if it does not exist");
	    	println!("list: lists the departments and their members.");
	    	continue;
	    }

        if input_slice == "list" {
	    	print_dir(&directory);
	    	continue;
	    }

	    match add_cmd.captures(input_slice) {
	    	Some(cap) => {
	    		let who = cap["who"].to_string();
	    		let dept = cap["department"].to_string();
	    		println!("Adding {} to {}", who, dept);
	    		add_to_directory(&mut directory, who, dept);
	    	},
	    	None => {
	    		println!("I did not understand '{}'", input_slice.to_string());
	    		continue;
	    	}
	    }

	   
	}
}
