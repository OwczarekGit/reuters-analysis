extern crate core;

use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use minidom::Element;

fn main() {
    let sgm_files = get_files_with_extension("reuters", "sgm");
    sgm_files.iter().for_each(|file| {
        if let Some(content) = read_sgm_file_content(file) {
            let root: Element = content.parse().unwrap();
            println!("{}", root.text());
        }
    });
}

fn read_sgm_file_content(file: &DirEntry) -> Option<String> {
    if let Some(content) = read_file_content(file){
        let mut combined: String = String::from("<ROOT xmlns=\"\">");
        combined.push_str(&content
            .replace("<!DOCTYPE lewis SYSTEM \"lewis.dtd\">","")
            .replace("&", "&amp;")
            .replace("\"", "&quot;")
            .replace("<", "&lt;")
            .replace(">", "&gt;"));
        combined.push_str("</ROOT>");
        return Some(combined);
    }
    None
}

fn read_file_content(file: &DirEntry) -> Option<String> {
    if let Ok(file_content) = fs::read_to_string(file.path()){
        return Some(file_content)
    }
    None
}

fn get_files_with_extension(directory: &str, extension: &str) -> Vec<DirEntry> {
    let mut file_list = vec![];
    if let Ok(files) =  Path::new(directory).read_dir(){
        files.for_each(|file| {
            if let Ok(file) = file {
                let path = file.path();
                if let Some(file_extension) = path.extension() {
                    if file_extension == extension {
                        file_list.push(file)
                    }
                }
            }
        })
    }
    file_list
}
