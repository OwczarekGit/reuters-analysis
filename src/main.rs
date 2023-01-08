extern crate core;

use std::cmp::{Ordering};
use std::collections::HashMap;
use std::{fs, io};
use std::io::Write;
use clap::{Parser, ValueEnum};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

mod article;
mod utils;
use crate::article::article::{Article};
use crate::article::element_type::ElementType;
use crate::utils::sgm_helper::get_articles_from_sgm;

#[derive(Parser, Debug)]
#[command(version)]
struct Config{
    directory: String,
    #[arg(short, long, default_value_t = 3)]
    k: usize,
    #[arg(short, long, default_value_t = 0.7)]
    ratio: f32,
    #[arg(short, long)]
    json_dump: Option<String>,
    #[arg(short, long, value_enum, default_value_t = Algorithm::EUCLIDEAN)]
    algorithm: Algorithm,
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Algorithm {
    EUCLIDEAN,
    MANHATTAN,
    CHEBYSHEV,
}

fn main() {

    let config = Config::parse();

    let ratio = config.ratio;

    let searched = vec![
        "usa".to_string(),
        "uk".to_string(),
        "west-germany".to_string(),
        "france".to_string(),
        "canada".to_string(),
        "japan".to_string()];

    let mut all_articles = vec![];
    for article in get_articles_from_sgm(config.directory.as_str()) {
        if article.places.len() == 1 && searched.contains(&article.places[0]) {
            all_articles.push(article)
        }
    }

    json_dump(&config, &all_articles);
    let (training_slice, testing_slice) = split_dataset(ratio, all_articles);
    print_general_params(&config, &training_slice, &testing_slice);
    classify_test_data_and_verify(testing_slice, training_slice, config);

}

fn print_general_params(config: &Config, training_slice: &Vec<Article>, testing_slice: &Vec<Article>) {
    println!("Testing Slice: {}, Training Slice: {}", testing_slice.len(), training_slice.len());
    println!("K: {}, Ratio: {}", config.k, config.ratio);
}

fn json_dump(config: &Config, all_reuters: &Vec<Article>) {
    if let Some(json_dump) = &config.json_dump {
        if let Ok(buf) = serde_json::to_string_pretty(&all_reuters) {
            if let Ok(_) = fs::write(json_dump, buf) {
                println!("Successfully saved article as JSON.");
            } else {
                panic!("Failed to write article to JSON file.");
            }
            return;
        }
    }
}

fn split_dataset(ratio: f32, articles: Vec<Article>) -> (Vec<Article>, Vec<Article>) {
    let slicing_point = (articles.len() as f32 * ratio) as usize;
    let (training_slice, testing_slice) = (articles[..slicing_point].to_vec(), articles[slicing_point..].to_vec());
    return (training_slice, testing_slice);
}

fn create_countries_map(articles: &Vec<Article>) -> HashMap<String, i32>{
    let mut map: HashMap<String, i32> = HashMap::new();
    articles.iter().for_each(|article| {
      map.insert(article.places[0].clone(), 0);
    });

    map
}

fn classify_test_data_and_verify(
    test_articles: Vec<Article>,
    train_articles: Vec<Article>,
    config: Config) {

    let test_articles_size = test_articles.len();
    let train_articles_size = train_articles.len();
    let mut counter: i32 = 0;
    let mut all_ok: i32 = 0;

    let mut true_positive:  HashMap<String, i32> = HashMap::new();
    let mut false_positive: HashMap<String, i32> = HashMap::new();
    let mut false_negative: HashMap<String, i32> = HashMap::new();
    let mut true_negative:  HashMap<String, i32> = HashMap::new();
    let all_countries_map:  HashMap<String, i32> = create_countries_map(&test_articles);

    for article in test_articles {

        let classification_result: String = classify_datapoint(&article, &train_articles, &config);
        let place = article.places[0].clone();
        let verification_result: bool = verify_classification(article, classification_result.clone());
        if verification_result == true {
            all_ok += 1;
            *true_positive.entry(classification_result.clone()).or_insert(1) += 1;
            for (key, _) in &all_countries_map {
                if *key != classification_result.clone() {
                    *true_negative.entry(key.clone()).or_insert(1) += 1;
                }
            }
        }
        else {
            *false_positive.entry(classification_result.clone()).or_insert(1) += 1;
            *false_negative.entry(place.clone()).or_insert(1) += 1;

        }
        counter += 1;
        print!("\rProgress: {:.2}%", (counter as f32 / test_articles_size as f32) *100f32);
        io::stdout().flush().unwrap();
    }

    let accuracy: f32 = all_ok as f32 / test_articles_size as f32;

    let mut precision_map: HashMap<String, f32> = HashMap::new();
    let mut recall_map: HashMap<String, f32> = HashMap::new();
    let mut fallout_map: HashMap<String, f32> = HashMap::new();

    for (key, _) in all_countries_map {
        let tp: f32    = *true_positive.get(key.clone().as_str()).unwrap_or(&0i32) as f32;
        let fp: f32    = *false_positive.get(key.clone().as_str()).unwrap_or(&0i32) as f32;
        let f_neg: f32 = *false_negative.get(key.clone().as_str()).unwrap_or(&0i32) as f32;
        let tn: f32    = *true_negative.get(key.clone().as_str()).unwrap_or(&0i32) as f32;

        let precision: f32 = if tp + fp == 0.0 {
            0f32
        } else {
            tp / (tp + fp)
        };

        precision_map.insert(key.clone(), precision);

        let recall: f32 = if tp + f_neg == 0.0 {
            0f32
        } else {
            tp / (tp + f_neg)
        };
        recall_map.insert(key.clone(), recall);

        let fall_out: f32 = if fp + tn == 0.0 {
            0f32
        } else {
            fp / (fp + tn)
        };
        fallout_map.insert(key.clone(), fall_out);
    }

    let result = Result{ 
        k: config.k, 
        split_ratio: 
        config.ratio, 
        testing_slice_size: test_articles_size,
        training_slice_size: train_articles_size,
        accuracy, 
        precisions: precision_map, 
        fallout: fallout_map, 
        recall: recall_map 
    };

    println!("\n{}",serde_json::to_string_pretty(&result).unwrap());
}

#[derive(Clone, Serialize, Deserialize)]
struct Result {
    k: usize,
    split_ratio: f32,
    testing_slice_size: usize,
    training_slice_size: usize,
    accuracy: f32,
    precisions: HashMap<String, f32>,
    fallout: HashMap<String, f32>,
    recall: HashMap<String, f32>
}

fn classify_datapoint(test_article: &Article, train_articles: &Vec<Article>, config: &Config) -> String {

    let mut distances: Vec<CalculatedArticle> = calculate_distances(test_article, train_articles, config.algorithm);
    let nearest_neighbors = find_k_nearest_neighbors(&mut distances, config.k as i32);

    return calculate_classification_result(nearest_neighbors);
}

fn calculate_classification_result(nearest_neighbors: Vec<Article>) -> String {
    let mut map:HashMap<String, i32> = HashMap::new();
    for neighbour in nearest_neighbors {
        let place = neighbour.places.get(0).unwrap().as_str();
        if map.contains_key(place) {
            let counter: i32 = *map.get(place).unwrap() + 1;
            map.insert(place.to_string(), counter);
        } else {
            map.insert(place.to_string(), 1);
        }
    }

    map.iter().max_by(|(_,a),(_,b)| {
        if a > b {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }).unwrap().0.to_string()
}
#[derive(Debug)]
struct CalculatedArticle {
    pub article: Article,
    pub distance: f32
}

fn find_k_nearest_neighbors(distances: &mut Vec<CalculatedArticle>, k: i32) -> Vec<Article> {

    distances.sort_by(|a, b| {
        if a.distance > b.distance {
            Ordering::Greater
        } else {
            Ordering::Less
        }

    });

    let mut y: Vec<Article> = vec![];
    for distance in distances {
        y.push(distance.article.clone());
    }


    return y[..k.min(y.len() as i32) as usize].to_vec();
}

fn calculate_distances(test_article: &Article, train_articles: &Vec<Article>, algorithm: Algorithm) -> Vec<CalculatedArticle> {
    train_articles.par_iter()
        .map(|article| {
            calculate_distance(test_article.clone(), &article, algorithm)
        }).collect()
}

fn calculate_distance(a: Article, b: &Article, algorithm: Algorithm) -> CalculatedArticle {
    let distance = match algorithm {
        Algorithm::EUCLIDEAN => a.distance_euclidean(&b),
        Algorithm::MANHATTAN => a.distance_manhattan(&b),
        Algorithm::CHEBYSHEV => a.distance_chebyshev(&b),
        // _ => panic!("INVALID ALGORITHM")
    };

    CalculatedArticle { article: b.clone(), distance }
}

fn verify_classification(article: Article, classification_result: String) -> bool {
    let x = if *article.places.get(0).unwrap() == classification_result {
        true
    } else {
        false
    };
    x
}
