extern crate core;

use std::borrow::Borrow;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use minidom::Element;
use regex::Regex;
use serde::{Deserialize, Serialize};
use crate::ElementType::{BODY, OTHER, PLACES, TOPICS};

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

    // for reuter in all_reuters {
        // println!("{:?}", reuter);
        // println!("{}", serde_json::to_string(&reuter).unwrap());
        println!("{}", serde_json::to_string(&all_reuters).unwrap());
    // }
}

macro_rules! clean {
    ($s:expr) => {
        $s.trim().to_string()
    };
}

enum ElementType{
    TOPICS(Vec<String>),
    PLACES(Vec<String>),
    BODY(String, String),
    OTHER,
}

impl ElementType {
    pub fn new(element: &Element) -> Self {
        match element.name() {
            "TOPICS" => Self::get_topics(element),
            "PLACES" => Self::get_places(element),
            "TEXT" => Self::get_text_details(element),
            _ => OTHER,
        }
    }

    fn get_topics(element: &Element) -> ElementType {
        let mut items = vec![];
        for child in element.children() {
            items.push(clean!(child.text()));
        }
        TOPICS(items)
    }

    fn get_places(element: &Element) -> ElementType {
        let mut items = vec![];
        for child in element.children() {
            items.push(clean!(child.text()));
        }
        PLACES(items)
    }

    fn get_text_details(element: &Element) -> ElementType {
        let mut title = String::new();
        let mut body = String::new();

        for child in element.children() {
            if child.is("TITLE","") {
                title = clean!(child.text());
            } else if child.is("BODY","") {
                body = clean!(child.text());
            }
        }

        BODY(title, body)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Reuters{
    topics: Vec<String>,
    places: Vec<String>,
    title: String,
    body:  String
}

impl Reuters{
    pub fn new(element: &Element) -> Self {

        let mut topics = vec![];
        let mut places = vec![];
        let mut title = String::new();
        let mut body = String::new();

        for child in element.children() {
            match ElementType::new(child){
                TOPICS(t) => topics = t,
                PLACES(p) => places = p,
                BODY(t,b) => (title, body) = (t, b),
                OTHER  => {},
            }
        }

        Self{ topics, places, title, body }
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
