use std::fs;

pub fn get_files(directory: String, format: &str) -> Vec<String>{
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

pub fn open_file(filename: &str) -> String{
    String::from_utf8(fs::read(filename.clone())
        .expect(&format!("{} was not found", filename.clone())))
        .unwrap()
}