use std::{fs, io::Write};

pub fn parse_parameters(){
    let mut args = std::env::args();
    if args.any(|x| x == "init".to_string()){
        let mut watch = fs::File::create("watch.json").unwrap();
        watch.write_all(
r#"{
    "to_watch": {
        "root": [
            "src/"
        ],
        "format": [
            
        ],
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