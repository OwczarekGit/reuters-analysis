use minidom::Element;
use serde::{Deserialize, Serialize};

use crate::article::element_type::ElementType;
use crate::article::element_type::ElementType::*;
use crate::article::text_traits::TextTraits;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub date: String,
    pub topics: Vec<String>,
    pub places: Vec<String>,
    pub people: Vec<String>,
    pub orgs: Vec<String>,
    pub exchanges: Vec<String>,
    pub unknown: String,
    pub title: String,
    pub dateline: String,
    pub body:  String
}

impl Article {
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

    pub fn distance_euclidean(&self, other: &Article) -> f32 {
        ((self.body.word_count() - other.body.word_count()).pow(2) +
            (self.body.characters_count() - other.body.characters_count()).pow(2) +
            (self.body.sentence_count() - other.body.sentence_count()).pow(2) +
            (self.title.word_count() - other.title.word_count()).pow(2) +
            (self.title.characters_count() - other.title.characters_count()).pow(2)) as f32
    }

    pub fn distance_manhattan(&self, other: &Article) -> f32 {
        ((self.body.word_count() - other.body.word_count()).abs() +
            (self.body.characters_count() - other.body.characters_count()).abs() +
            (self.body.sentence_count() - other.body.sentence_count()).abs() +
            (self.title.word_count() - other.title.word_count()).abs() +
            (self.title.characters_count() - other.title.characters_count()).abs()) as f32
    }

    pub fn distance_chebyshev(&self, other: &Article) -> f32 {
        let mut distances = vec![];

        distances.push((       self.body.word_count() - other.body.word_count()       ).abs());
        distances.push(( self.body.characters_count() - other.body.characters_count() ).abs());
        distances.push((   self.body.sentence_count() - other.body.sentence_count()   ).abs());
        distances.push((      self.title.word_count() - other.title.word_count()      ).abs());
        distances.push((self.title.characters_count() - other.title.characters_count()).abs());

        *(distances.iter().max().unwrap_or(&0)) as f32
    }
}