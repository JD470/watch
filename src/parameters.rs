use std::{str::FromStr, fs, io::Write};

pub fn parse_parameters(){
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&String::from_str("init").unwrap()){
        let mut watch = fs::File::create("watch.json").unwrap();
        watch.write_all(
r#"{
    "watch": {
        "root": "src/",
        "format": "",
        "commands":[
            
        ]
    },
    "keys": {
        "events":[
            {
                "keys": [],
                "commands": [
                    
                ]
            }
        ]
    }
}"#.as_bytes()).unwrap();
        println!("A new watch file has been created!");
        std::process::exit(0);
    }
}