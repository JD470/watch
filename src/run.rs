use std::{process::Command, time::Duration};
use device_query::{Keycode};

use crate::files::*;
use crate::keys::*;
use crate::json_struct::*;

pub fn run(){
    let file = open_file("watch.json");

    let watch: Watch = nanoserde::DeJson::deserialize_json(file.as_str()).expect("JSON was not well-formatted");
    let to_watch = watch.to_watch;
    let keys = watch.keys;
    
    let mut directory = get_all_files(&to_watch.root, &to_watch.format);
    let mut times: Vec<u64> = get_times(&directory);

    let key_events = keys.events;
    
    loop{
        // Watch
        directory = get_all_files(&to_watch.root, &to_watch.format);
        let temp: Vec<u64> = get_times(&directory);

        if compare_times(&times, &temp) {
            let changed_file = get_changed_file_name(&directory, &times);
            times = temp;
            execute_commands(&to_watch.commands, &changed_file);
        }

        // Key events
        for i in &key_events{
            let mut keys_pressed: Vec<Keycode> = get_keys_pressed();
            if keys_pressed.len() != i.keys.len(){
                break;
            }

            let key_event: Vec<Keycode> = string_list_to_keycode_list(&i.keys);
            keys_pressed = get_matching_keycodes(keys_pressed, &key_event);
            
            if keys_pressed.len() == key_event.len(){
                execute_commands(&i.commands, "");
            }
        }

        // Wait 50ms
        std::thread::sleep(Duration::from_millis(50));
    }
}

fn execute_commands(commands: &Vec<String>, file: &str){
    let str = if !file.is_empty() {file} else {"!{$}"};
    
    for i in commands{
        print!("{}",
            String::from_utf8
            (
                Command::new("cmd")
                .args(
                [
                    "/C", &i.to_string().replace("!{$}", str)
                ]
                ).output().unwrap().stdout
            ).unwrap()
        );
    }
}