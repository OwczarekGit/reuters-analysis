extern crate core;

use std::cmp::{Ordering};
use std::collections::HashMap;
use std::{fs, io};
use std::io::Write;
use std::mem::transmute;
use clap::{Parser, ValueEnum};
use rayon::prelude::*;
mod article;
mod utils;
use crate::article::article::{Article};
use crate::article::element_type::ElementType;
use crate::utils::cleaner::Cleaner;
use crate::utils::sgm_helper::get_articles_from_sgm;

#[derive(Parser, Debug)]
#[command(version)]
struct Config{
    directory: String,
    #[arg(short, long, default_value_t = 3)]
    k: usize,
    #[arg(short, long, default_value_t = 0.7)]
    ratio: f32,
    #[arg(short, long, default_value_t = 100)]
    test_size: usize,
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
    let k = config.k;
    let ratio = config.ratio;

    let mut all_articles= vec![];
    for article in get_articles_from_sgm(config.directory.as_str()) {
        if article.places.len() > 0 {
            all_articles.push(article)
        }
    }

    json_dump(&config, &all_articles);
    let (training_slice, testing_slice) = split_dataset(ratio, all_articles);
    print_general_params(&config, &training_slice, &testing_slice);
    classify_test_data_and_verify(testing_slice, training_slice, k as i32);

}

fn print_general_params(config: &Config, training_slice: &Vec<Article>, testing_slice: &Vec<Article>) {
    println!("Testing Slice: {}, Training Slice: {}", testing_slice.len(), training_slice.len());
    println!("K: {}, Ratio: {}, Test Size: {}\n", config.k, config.ratio, config.test_size);
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

fn classify_test_data_and_verify(
    test_articles: Vec<Article>,
    train_articles: Vec<Article>, k: i32) {

    let test_articles_size = test_articles.len();
    let mut all_ok: i32 = 0;
    let mut all_not_ok: i32 = 0;
    let mut place_ok: HashMap<String, i32> = HashMap::new();
    let mut place_not_ok: HashMap<String, i32> = HashMap::new();
    let mut counter: i32 = 0;
    for article in test_articles {
        let classification_result = classify_datapoint(&article, &train_articles, k);
        let place = article.places[0].clone();
        let verification_result: bool = verify_classification(article, classification_result);
        // println!("Ver. Result no. {} : {}", counter, verification_result);
        counter += 1;
        if (verification_result == true) {
            all_ok += 1;
            if(place_ok.contains_key(place.as_str())) {
                let counter = place_ok.get(place.as_str()).unwrap() + 1;
                place_ok.insert(place.to_string(), counter);
            } else {
                place_ok.insert(place.to_string(), 1);
            }
        }
            else {
            all_not_ok += 1;
            if(place_not_ok.contains_key(place.as_str())) {
                let counter = place_not_ok.get(place.as_str()).unwrap() + 1;
                place_not_ok.insert(place.to_string(), counter);
            } else {
                place_not_ok.insert(place.to_string(), 1);
            }
        }
        print!("\rProgress: {:.2}%", (counter as f32 / test_articles_size as f32) *100f32);
        io::stdout().flush().unwrap();
    }
    println!();
    println!("{} out of {} classified correctly. (Accuracy: {:.2}% )", all_ok, test_articles_size, (all_ok as f32 / test_articles_size as f32) * 100f32 );
    println!("{} out of {} classified incorrectly.", all_not_ok, test_articles_size);
    println!("Places ok: {:?}", place_ok);
    println!("Places not ok: {:?}", place_not_ok);

}

fn classify_datapoint(test_article: &Article, train_articles: &Vec<Article>, k: i32) -> String {

    let mut distances: Vec<CalculatedArticle> = calculate_distances(test_article, train_articles);
    let nearest_neighbors = find_k_nearest_neighbors(&mut distances, k);
    let mut temp: Vec<String> = vec![];

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

fn calculate_distances(test_article: &Article, train_articles: &Vec<Article>) -> Vec<CalculatedArticle> {
    train_articles.par_iter()
        .map(|article| {
            calculate_distance(test_article.clone(), &article)
        }).collect()
}

fn calculate_distance(a: Article, b: &Article) -> CalculatedArticle {
    let distance = a.distance_euclidean(&b);
    return CalculatedArticle{
        article: b.clone(),
        distance
    };
}

fn verify_classification(article: Article, classification_result: String) -> bool {
    let x = if *article.places.get(0).unwrap() == classification_result {
        true
    } else {
        false
    };
    x
}



// for testing in testing_slice {
//     let samples: Vec<&Article> = training_slice.choose_multiple(&mut rand::thread_rng(), testing_items).collect();
//     let mut distances = vec![];
//     for sample in samples {
//
//         let distance = match config.algorithm {
//             Algorithm::EUCLIDEAN => testing.distance_euclidean(sample),
//             Algorithm::MANHATTAN => testing.distance_manhattan(sample),
//             Algorithm::CHEBYSHEV => testing.distance_chebyshev(sample),
//         };
//
//         distances.push((sample, distance));
//         distances.sort_by(|(_, a), (_, b) | {
//             if a > b {
//                 Ordering::Greater
//             } else {
//                 Ordering::Less
//             }});
//
//     }
//     println!("→ {:?}", testing.numbers());
//     let k_checked = &distances[..min(k, distances.len())];
//     for (kc,d) in k_checked {
//         println!("  {:?} ←→ {}", kc.numbers(), d);
//     }
//     println!("-----------")
//
// }
