use std::{process::Command, time::Duration};

use files::*;
use serde_derive::Deserialize;

mod files;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
struct ToWatch {
    root: String,
    commands: Vec<String>,
}

fn main() {
    let file = open_file("watch.json");
    
    let to_watch: ToWatch = serde_json::from_str(file.as_str()).expect("JSON was not well-formatted");
    let root = to_watch.root;
    let commands = to_watch.commands;
    let mut directory = get_all_files(&root, ".rs");
    
    let mut times: Vec<u64> = get_times(&directory);

    loop{
        directory = get_all_files(&root, ".rs");
        let temp: Vec<u64> = get_times(&directory);
        if compare_vecs(&times, &temp) {
            times = temp;
            execute_args(&commands);
        }
        
        std::thread::sleep(Duration::from_millis(50));
    }
}

fn compare_vecs(temp1: &Vec<u64>, temp2: &Vec<u64>) -> bool{
    if temp1.len() != temp2.len(){return true;}
    for i in 0..temp1.len(){
        if temp1[i] != temp2[i] {
            return true;
        }
    }
    false
}

fn execute_args(commands: &Vec<String>){
    for i in commands{
        println!("{}",
            String::from_utf8(
            Command::new("cmd").args([
                "/C", &i.to_string()
            ])
            .output().
            unwrap().
            stdout).unwrap()

        )
    }
}