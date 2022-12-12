extern crate core;

use std::borrow::Borrow;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use minidom::Element;
use regex::Regex;

fn main() {
    let sgm_files = get_files_with_extension("reuters", "sgm");
    let mut all_reuters: Vec<Reuters> = vec![];
    sgm_files.iter().for_each(|file| {
        if let Some(content) = read_sgm_file_content(file) {
            if let Ok(root) = content.parse::<Element>(){
                root.children().for_each(|child|{
                    if child.is("REUTERS", "") {
                        all_reuters.push(Reuters::new(&child));
                    }
                });
            }
        }
    });

    for reuter in all_reuters {
        println!("{:?}", reuter);
    }
}

#[derive(Debug)]
struct Reuters{
    topics: Vec<String>,
    places: Vec<String>,
    title: String,
    body:  String
}

impl Reuters{
    pub fn new(element: &Element) -> Self{
        let topics = Reuters::get_child(&element, "TOPICS");
        let places = Reuters::get_child(&element, "PLACES");
        let (title, body) = Reuters::get_text_details(&element);

        Self{ topics, places, title, body }
    }

    fn get_text_details(element: &Element) -> (String, String) {
        let mut title_val = String::new();
        let mut body_val = String::new();
        element.children().for_each(|child|{
            if child.is("TEXT","") {
                child.children().for_each(|text_child|{
                    if text_child.is("BODY","") {
                        body_val = text_child.text().trim().to_string();
                    } else if text_child.is("TITLE","") {
                        title_val = text_child.text().trim().to_string();
                    }
                })
            }
        });

        (title_val, body_val)
    }

    fn get_child(element: &Element, node: &str) -> Vec<String>{
        let mut items = vec![];
        element.children().for_each(|child| {
            if child.is(node, "") {
                child.children().for_each(|place| {
                    items.push(place.text().trim().to_string());
                });
            }
        });
        items
    }
}

fn read_sgm_file_content(file: &DirEntry) -> Option<String> {
    if let Some(content) = read_file_content(file){
        let regex = Regex::new("&#.*?;").unwrap();
        let mut combined: String = String::from("<articles xmlns=\"\">");
        let removed_doctype = &content.replace("<!DOCTYPE lewis SYSTEM \"lewis.dtd\">","");
        let fixed = regex.replace_all(removed_doctype,"");
        combined.push_str(fixed.borrow());
        combined.push_str("</articles>");
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
