use std::{fs, os::windows::prelude::MetadataExt};

use crate::json_struct::{ToWatch};

pub fn get_folders(directory: &String) -> Vec<String>{
    let mut folders: Vec<String> = vec![];

    for file in fs::read_dir(directory).unwrap(){
        let current_file = file.unwrap();

        if current_file.path().is_dir(){
            if !directory.ends_with('/') {
                folders.push(format!("{}/{}", directory,current_file.file_name().to_str().unwrap()));
            }

            else{
                folders.push(format!("{}{}", directory,current_file.file_name().to_str().unwrap()));
            }
        }
    }

    folders
}

pub fn get_all_folders(root: &String) -> Vec<String>{
    let mut ret_folders: Vec<String> = vec![];
    let mut folders: Vec<String> = get_folders(root);
    let mut temp: Vec<String>;

    loop {
        ret_folders.extend(folders.clone());

        temp = folders;
        folders = vec![];

        for directory in temp{
            folders.extend(get_folders(&directory));
        }

        if folders.is_empty() {
            return ret_folders;
        }
    }
}

pub fn get_files(directory: &String, format: &str) -> Vec<String>{
    fs::read_dir(directory).unwrap().into_iter()
        .map(|x| x.unwrap().path().display().to_string())
        .filter(|x| x.ends_with(format)).collect()
}

pub fn get_all_files(root: &String, format: &str) -> Vec<String>{
    let mut files: Vec<String> = get_files(root, format);
    let folders = get_all_folders(root);

    for folder in folders{
        for file in get_files(&folder, format){
            files.push(file.replace('\\', "/"));
        }
    }

    files
}

pub fn get_times(files: &Vec<String>) -> Vec<u64>{
    files.iter().map(|i| fs::metadata(i).unwrap().last_write_time()).collect()
}

pub fn get_changed_file_name(files: &Vec<String>, past_times: &Vec<u64>) -> String{
    let mut file: String = String::new();
    
    for i in 0..files.len(){
        if fs::metadata(files.get(i).unwrap()).unwrap().last_write_time() != past_times[i] {
            file = files.get(i).unwrap().to_string();
        }
    }

    file
}

pub fn get_files_info(to_watch: &ToWatch) -> (Vec<String>, Vec<u64>) {
    let mut ret_files: Vec<String> = Vec::new();

    for root in &to_watch.root{
        for format in &to_watch.format{
            ret_files.extend(get_all_files(root, format));
        }
    }
    let ret_times = get_times(&ret_files);

    (ret_files, ret_times)
}

pub fn open_file(filename: &str) -> String{
    fs::read_to_string(filename).expect(&format!("{} was not found", filename))
}

pub fn compare_times(temp1: &Vec<u64>, temp2: &Vec<u64>) -> bool{
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