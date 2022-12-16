extern crate core;

use std::borrow::Borrow;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use minidom::Element;
use regex::Regex;
use serde::{Deserialize, Serialize};
use crate::ElementType::{BODY, DATE, EXCHANGES, ORGS, OTHER, PEOPLE, PLACES, TOPICS, UNKNOWN};

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

    // println!("{}",all_reuters.len());
    println!("{}", serde_json::to_string_pretty(&all_reuters).unwrap());
}

trait Cleaner {
    fn clean(&self) -> String;
}

impl Cleaner for String {
    fn clean(&self) -> String {
        self.trim().to_string()
    }
}

enum ElementType{
    DATE(String),
    TOPICS(Vec<String>),
    PLACES(Vec<String>),
    PEOPLE(Vec<String>),
    ORGS(Vec<String>),
    EXCHANGES(Vec<String>),
    // COMPANIES(Vec<String>), Skip - always empty
    UNKNOWN(String),
    BODY(String, String, String),
    OTHER,
}

impl ElementType {
    pub fn new(element: &Element) -> Self {
        match element.name() {
            "DATE" => DATE(element.text().clean()),
            "TOPICS" => Self::get_topics(element),
            "PLACES" => Self::get_places(element),
            "PEOPLE" => Self::get_people(element),
            "ORGS" => Self::get_orgs(element),
            "EXCHANGES" => Self::get_exchanges(element),
            "UNKNOWN" => UNKNOWN(element.text().clean()),
            "TEXT" => Self::get_text_details(element),
            _ => OTHER,
        }
    }

    fn get_exchanges(element: &Element) -> ElementType {
        let mut items = vec![];
        for child in element.children() {
            items.push(child.text().clean());
        }
        EXCHANGES(items)
    }

    fn get_orgs(element: &Element) -> ElementType {
        let mut items = vec![];
        for child in element.children() {
            items.push(child.text().clean());
        }
        ORGS(items)
    }

    fn get_people(element: &Element) -> ElementType {
        let mut items = vec![];
        for child in element.children() {
            items.push(child.text().clean());
        }
        PEOPLE(items)
    }

    fn get_topics(element: &Element) -> ElementType {
        let mut items = vec![];
        for child in element.children() {
            items.push(child.text().clean());
        }
        TOPICS(items)
    }

    fn get_places(element: &Element) -> ElementType {
        let mut items = vec![];
        for child in element.children() {
            items.push(child.text().clean());
        }
        PLACES(items)
    }

    fn get_text_details(element: &Element) -> ElementType {
        let mut title = String::new();
        let mut dateline = String::new();
        let mut body = String::new();

        for child in element.children() {
            if child.is("TITLE","") {
                title = child.text().clean();
            } else if child.is("DATELINE", "") {
                dateline = child.text().clean();
            } else if child.is("BODY","") {
                body = child.text().clean();
            }
        }

        BODY(title, body, dateline)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Reuters{
    date: String,
    topics: Vec<String>,
    places: Vec<String>,
    people: Vec<String>,
    orgs: Vec<String>,
    exchanges: Vec<String>,
    unknown: String,
    title: String,
    dateline: String,
    body:  String
}

impl Reuters{
    pub fn new(element: &Element) -> Self {

        let mut topics = vec![];
        let mut date = String::new();
        let mut places = vec![];
        let mut people = vec![];
        let mut orgs = vec![];
        let mut exchanges = vec![];
        let mut unknown = String::new();
        let mut title = String::new();
        let mut dateline = String::new();
        let mut body = String::new();

        for child in element.children() {
            match ElementType::new(child){
                DATE(d) => date = d,
                TOPICS(t) => topics = t,
                PLACES(p) => places = p,
                PEOPLE(p) => people = p,
                ORGS(o) => orgs = o,
                EXCHANGES(e) => exchanges = e,
                UNKNOWN(u) => unknown = u,
                BODY(t,b, d) => (title, body, dateline) = (t, b, d),
                OTHER  => {},
            }
        }

        Self{ date, topics, places, people, orgs, exchanges, unknown, title, dateline, body }
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
