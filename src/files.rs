use std::{fs, os::windows::prelude::MetadataExt};

pub fn get_files(directory: &String, format: &str) -> Vec<String>{
    let paths = fs::read_dir(directory).unwrap();
    let lang_format = format;
    let mut ps: Vec<String> = vec![];

    for path in paths {
        let p = path.unwrap().path().display().to_string();

        if p.ends_with(lang_format) {
            ps.push(p);
        }
    }
    ps
}

pub fn get_folders(directory: &String) -> Vec<String>{
    let mut temp: Vec<String> = vec![];

    for file in fs::read_dir(directory).unwrap(){
        let temp1 = file.unwrap();

        if temp1.path().is_dir(){

            if !directory.ends_with("/") {
                temp.push(format!("{}/{}", directory,temp1.file_name().to_str().unwrap().to_string()));
            }

            else{
                temp.push(format!("{}{}", directory,temp1.file_name().to_str().unwrap().to_string()));
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

        temp1 = temp.clone();
        temp = vec![];

        for i in temp1{
            let folders = get_folders(&i);

            for j in folders{
                temp.push(j.clone());
            }
        }

        if temp.is_empty() {
            return ret;
        }
    }
}

pub fn get_all_files(root: &String, format: &str) -> Vec<String>{
    let mut ret: Vec<String> = get_files(root, format);
    let folders = get_all_folders(root);

    for i in folders{
        for j in get_files(&i, format){
            ret.push(j.replace("\\", "/"));
        }
    }

    ret
}

pub fn open_file(filename: &str) -> String{
    String::from_utf8(
        fs::read(filename.clone())
            .expect(&format!("{} was not found", filename.clone())
        )
    )
        .unwrap()
}

pub fn get_times(files: &Vec<String>) -> Vec<u64>{
    files.into_iter().map(|i| fs::metadata(i).unwrap().last_write_time()).collect()
}