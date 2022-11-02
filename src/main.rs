use std::{process::Command, time::Duration, str::FromStr, fs, io::Write};

mod files;
mod json_struct;
mod keys;

use device_query::{Keycode};
use files::*;
use keys::*;
use json_struct::*;

#[allow(unused_must_use)] // For the init function("watch.write_all()" to be precise)

fn main(){
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
}"#.as_bytes());
        return;
    }

    let file = open_file("watch.json");
    
    let to_watch: Watch = serde_json::from_str(file.as_str()).expect("JSON was not well-formatted");
    let root = to_watch.watch.root;
    let format = to_watch.watch.format;
    let commands = to_watch.watch.commands;
    
    let mut directory = get_all_files(&root, &format);
    let mut times: Vec<u64> = get_times(&directory);

    let key_events = to_watch.keys.events;
    
    loop{
        // Watch
        directory = get_all_files(&root, &format);
        let temp: Vec<u64> = get_times(&directory);

        if compare_times(&times, &temp) {
            times = temp;
            execute_args(&commands);
        }

        // Key events
        for i in &key_events{
            let mut keys_pressed: Vec<Keycode> = get_keys_pressed();
            if keys_pressed.len() != i.clone().keys.len(){
                break;
            }
            let key_event: Vec<Keycode> = string_list_to_keycode_list(i.clone().keys);
            keys_pressed = get_matching_keycodes(keys_pressed, &key_event);
            
            if keys_pressed.len() == key_event.len(){
                execute_args(&i.clone().commands);
            }
        }

        // Wait 50ms
        std::thread::sleep(Duration::from_millis(50));
    }
}

fn execute_args(commands: &Vec<String>){
    for i in commands{
        print!("{}",
            String::from_utf8
            (
                Command::new("cmd")
                .args(
                [
                    "/C", &i.to_string()
                ]
                ).output().unwrap().stdout
            ).unwrap()
        );
    }
}