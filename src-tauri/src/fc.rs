use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct FileInfo {
    name: String,
    directory: bool,
    path: String,
    children:Vec<FileInfo>,
}

pub fn read_directory(dir_path: &str) -> Vec<FileInfo>{
    let new_path = Path::new(dir_path);
    let mut filetree:Vec<FileInfo> = Vec::new();
    if new_path.is_dir() { 
        let paths = fs::read_dir(new_path).unwrap();
        for path in paths {
            let path_unwrap = path.unwrap();
            let meta = path_unwrap.metadata();
            let meta_unwrap = meta.unwrap();

            let filename = match path_unwrap.file_name().into_string() {
                Ok(str) => str,
                Err(_error) => String::from("ERROR"),
            };

            let file_path = dir_path.to_owned() + &filename;

            let mut directory = false;
            let mut children = Vec::new();
            if meta_unwrap.is_dir() && pathblacklist(&file_path){
                directory = true;
                let folder_path = dir_path.to_owned() + &filename + "/";
                children = read_directory(&folder_path);
            }

            let new_file_info = FileInfo {
                name: filename,
                directory: directory,
                path: file_path,
                children,
            };

            filetree.push(new_file_info);
        }
    }
    filetree
}

fn pathblacklist(folder_path: &str) -> bool{
    let binding = fs::read_to_string("./src/blacklist.txt").unwrap();
    let blacklist:Vec<&str>= binding.split(",").collect();
    if blacklist.contains(&folder_path) {
        return false;
    }
    return  true;
}
