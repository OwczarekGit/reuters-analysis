extern crate core;

use std::borrow::Borrow;
use std::cmp::{min, Ordering};
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

    let k = 3;

    let split = (all_reuters.len() as f32 * 0.9) as usize;

    let training_slice = &all_reuters[0..split];
    let testing_slice = &all_reuters[split..];

    println!("Training: {}, Testing: {}", training_slice.len(), testing_slice.len());

    let trained: Vec<Vec5> = training_slice.iter().map(|r| Vec5::new(r)).collect();

    testing_slice.iter().for_each(|test| {
        let mut closest = vec![];
        trained.iter().for_each(|train| {
            let dist = train.distance(&Vec5::new(test));
            closest.push(Vec5Distance::new(train.clone().to_owned(), dist as u32));
            closest.sort();

            println!("Tested: {:?}", Vec5::new(&test));
            let max = closest.len();
            let slice = &closest[0..min(max, k)];
            slice.iter().for_each(|item| println!("{:?}", item));
            println!("------------------")
        });
    })

    // let vectors: Vec<Vec5> = all_reuters.iter().map(|reut| {
    //    Vec5::new(reut)
    // }).collect();
    //
    // let slice = vectors[];
    //
    //
    // for vector in vectors {
    //     println!("{},{},{},{},{}",
    //              vector.body_word_count,
    //              vector.body_char_count,
    //              vector.body_sentence_count,
    //              vector.title_word_count,
    //              vector.title_char_count);
    // }

    // let mut p = HashMap::<String, u32>::new();
    // for reuter in all_reuters {
    //     for place in reuter.places {
    //         let exist = p.get(place.as_str());
    //         if let Some(exist) = exist {
    //             p.insert(place.to_string(), exist+1);
    //         }else {
    //             p.insert(place.to_string(), 1);
    //         }
    //     }
    //
    //     // let (wc, cc, sc) = (reuter.word_count(), reuter.char_count(), reuter.sentence_count());
    //     // println!("{}", reuter.date);
    //     // println!("WC:{}, CC:{}, SC:{}", reuter.word_count(), reuter.char_count(), reuter.sentence_count());
    // }

    // for pp in p {
    //     println!("{} : {}",pp.0, pp.1);
    // }

    // println!("{}",all_reuters.len());
    // println!("{}", serde_json::to_string_pretty(&all_reuters).unwrap());
}

#[derive(Debug, Eq)]
struct Vec5Distance {
    vec: Vec5,
    dist: u32,
}

impl Vec5Distance {
    pub fn new(vec: Vec5, dist: u32) -> Self {
        Self {vec, dist}
    }
}

impl Ord for Vec5Distance {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.dist > other.dist {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    }
}

impl PartialOrd for Vec5Distance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Vec5Distance {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Vec5 {
    body_word_count: u32,
    body_char_count: u32,
    body_sentence_count: u32,
    title_word_count: u32,
    title_char_count: u32,
}

impl Vec5 {
    pub fn new(reut: &Reuters) -> Self {
        Self {
            body_word_count: reut.word_count(),
            body_char_count: reut.char_count(),
            body_sentence_count: reut.sentence_count(),
            title_word_count: reut.title_word_count(),
            title_char_count: reut.title_char_count(),
        }
    }

    pub fn distance(&self, other: &Vec5) -> u32 {
        (self.body_word_count - other.body_word_count).pow(2) +
            (self.body_char_count - other.body_char_count).pow(2) +
            (self.body_sentence_count - other.body_sentence_count).pow(2) +
            (self.title_word_count - other.title_word_count).pow(2) +
            (self.title_char_count - other.title_char_count).pow(2)
    }
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

    fn title_word_count(&self) -> u32 {
        self.title.split(" ").count() as u32
    }

    fn title_char_count(&self) -> u32 {
        self.title.chars().collect::<Vec<char>>().len() as u32
    }

    fn word_count(&self) -> u32 {
        self.body.split(" ").count() as u32
    }

    fn char_count(&self) -> u32 {
        self.body.chars().collect::<Vec<char>>().len() as u32
    }

    fn sentence_count(&self) -> u32 {
        self.body.split(".").count() as u32
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
