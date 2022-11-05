use std::{fs, os::windows::prelude::MetadataExt};

use crate::json_struct::{ToWatch};

pub fn get_folders(directory: &String) -> Vec<String>{
    let mut temp: Vec<String> = vec![];

    for file in fs::read_dir(directory).unwrap(){
        let temp1 = file.unwrap();

        if temp1.path().is_dir(){
            if !directory.ends_with('/') {
                temp.push(format!("{}/{}", directory,temp1.file_name().to_str().unwrap()));
            }

            else{
                temp.push(format!("{}{}", directory,temp1.file_name().to_str().unwrap()));
            }
        }
    }
    temp
}

pub fn get_all_folders(root: &String) -> Vec<String>{
    let mut ret: Vec<String> = vec![];
    let mut temp: Vec<String> = get_folders(root);
    let mut temp1: Vec<String>;

    loop {
        for i in temp.clone(){
            ret.push(i);
        }

        temp1 = temp;
        temp = vec![];

        for i in temp1{
            let folders = get_folders(&i);

            for j in folders{
                temp.push(j);
            }
        }

        if temp.is_empty() {
            return ret;
        }
    }
}

pub fn get_files(directory: &String, format: &str) -> Vec<String>{
    fs::read_dir(directory).unwrap().into_iter()
        .map(|x| x.unwrap().path().display().to_string())
        .filter(|x| x.ends_with(format)).collect()
}

pub fn get_all_files(root: &String, format: &str) -> Vec<String>{
    let mut ret: Vec<String> = get_files(root, format);
    let folders = get_all_folders(root);

    for i in folders{
        for j in get_files(&i, format){
            ret.push(j.replace('\\', "/"));
        }
    }

    ret
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
    for i in &to_watch.root{
        for j in &to_watch.format{
            ret_files.extend(get_all_files(&i, &j));
        }
    }
    let ret_times = get_times(&ret_files);
    (ret_files, ret_times)
}

pub fn open_file(filename: &str) -> String{
    String::from_utf8(
        fs::read(filename)
            .expect(&format!("{} was not found", filename)
        )
    ).unwrap()
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