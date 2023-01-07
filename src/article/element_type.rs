use crate::{ElementType::*};
use minidom::Element;
use crate::utils::cleaner::Cleaner;

pub enum ElementType
{
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