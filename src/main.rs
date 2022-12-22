extern crate core;

use std::borrow::Borrow;
use std::cmp::{min, Ordering};
use std::fs;
use std::fs::{DirEntry};
use std::path::Path;
use minidom::Element;
use regex::Regex;
use rand::seq::SliceRandom;
use clap::Parser;
use serde::{Deserialize, Serialize};
// use rayon::prelude::*;
use crate::ElementType::{BODY, DATE, EXCHANGES, ORGS, OTHER, PEOPLE, PLACES, TOPICS, UNKNOWN};

#[derive(Parser, Debug)]
#[command(version)]
struct Config {
    directory: String,
    #[arg(short, long, default_value_t = 3)]
    k: usize,
    #[arg(short, long, default_value_t = 0.7)]
    ratio: f32,
    #[arg(short, long, default_value_t = 100)]
    test_size: usize,
    #[arg(short, long)]
    json_dump: Option<String>,
}


fn main() {

    let config: Config = Config::parse();
    let all_reuters = get_reuters_from_sgm_files(config.directory.as_str());

    if let Some(json_dump) = config.json_dump {
        if let Ok(buf) = serde_json::to_string_pretty(&all_reuters) {
            if let Ok(_) = fs::write(json_dump, buf) {
                println!("Successfully saved reuters as JSON.");
            } else {
                panic!("Failed to write reuters to JSON file.");
            }
            return;
        }
    }

    let testing_items = config.test_size;
    let k = config.k;
    let ratio = config.ratio;
    let split_ratio = (all_reuters.len() as f32 * ratio) as usize;
    let (training_slice, testing_slice) = (&all_reuters[..split_ratio], &all_reuters[split_ratio..]);

    println!("Testing Slice: {}, Training Slice: {}", testing_slice.len(), training_slice.len());
    println!("K: {}, Ratio: {}, Test Size: {}\n", config.k, config.ratio, config.test_size);
    for testing in testing_slice {
        let samples: Vec<&Reuters> = training_slice.choose_multiple(&mut rand::thread_rng(), testing_items).collect();
        let mut distances = vec![];
        for sample in samples {
            let distance = testing.distance(sample);
            distances.push((sample, distance));
            distances.sort_by(|(_, a), (_, b) | {
                if a > b {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }});

        }
        println!("→ {:?}", testing.numbers());
        let k_checked = &distances[..min(k, distances.len())];
        for (kc,d) in k_checked {
            println!("  {:?} ←→ {}", kc.numbers(), d);
        }
        println!("-----------")

    }
}

trait TextTraits {
    fn word_count(&self) -> i32;
    fn sentence_count(&self) -> i32;
    fn characters_count(&self) -> i32;
}

impl TextTraits for String {
    fn word_count(&self) -> i32 {
        self.split(" ").count() as i32
    }

    fn sentence_count(&self) -> i32 {
        self.split(".").count() as i32
    }

    fn characters_count(&self) -> i32 {
        self.chars().collect::<Vec<char>>().len() as i32
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

    pub fn distance(&self, other: &Reuters) -> f32 {
        (((self.body.word_count() - other.body.word_count()).pow(2) +
        (self.body.characters_count() - other.body.characters_count()).pow(2) +
        (self.body.sentence_count() - other.body.sentence_count()).pow(2) +
        (self.title.word_count() - other.title.word_count()).pow(2) +
        (self.title.characters_count() - other.title.characters_count()).pow(2)) as f32).sqrt()
    }

    pub fn numbers(&self) -> (i32, i32, i32, i32, i32) {
        (self.body.word_count(),
         self.body.characters_count(),
         self.body.sentence_count(),
         self.title.word_count(),
         self.title.characters_count())
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

fn get_reuters_from_sgm_files(dir :&str) -> Vec<Reuters> {
    let sgm_files = get_files_with_extension(dir, "sgm");
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
    all_reuters
}
