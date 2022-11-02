use std::{process::Command, time::Duration, str::FromStr};

mod files;
mod json_struct;

use device_query::{DeviceState, DeviceQuery, Keycode};
use files::*;
use json_struct::*;

fn main(){
    let file = open_file("watch.json");
    
    let to_watch: watch = serde_json::from_str(file.as_str()).expect("JSON was not well-formatted");
    let root = to_watch.watch.root;
    let format = to_watch.watch.format;
    let commands = to_watch.watch.commands;
    
    let mut directory = get_all_files(&root, &format);
    let mut times: Vec<u64> = get_times(&directory);

    let key_events = to_watch.keys.events;
    
    loop{
        directory = get_all_files(&root, &format);
        let temp: Vec<u64> = get_times(&directory);

        if compare_times(&times, &temp) {
            times = temp;
            execute_args(&commands);
        }

        for i in &key_events{
            let mut keys_pressed: Vec<Keycode> = get_keys_pressed();
            if keys_pressed.len() != i.clone().keys.len(){
                break;
            }
            let mut key_event: Vec<Keycode> = vec![];
            for j in i.clone().keys{
                key_event.push(Keycode::from_str(j.as_str()).unwrap());
            }
            keys_pressed = keys_pressed.into_iter().filter(|x| key_event.contains(x)).collect::<Vec<Keycode>>();
            if keys_pressed.len() == key_event.len(){
                execute_args(&i.clone().commands);
            }
        }

        std::thread::sleep(Duration::from_millis(50));
    }
}

fn get_keys_pressed() -> Vec<Keycode>{
    DeviceState::new().get_keys()
}

fn compare_times(temp1: &Vec<u64>, temp2: &Vec<u64>) -> bool{
    if temp1.len() != temp2.len(){
        return true;
    }

    for i in 0..temp1.len(){
        if temp1[i] != temp2[i] {
            return true;
        }
    }

    false
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