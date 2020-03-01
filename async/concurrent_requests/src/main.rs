// Make my own version with it.

// Code from: http://patshaughnessy.net/2020/1/20/downloading-100000-files-using-async-rust

use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use futures::stream::StreamExt;

use serde_json::Value;

fn read_lines(file_name: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    Ok(
        reader.lines().filter_map(Result::ok).collect()
    )
}

#[derive(Debug)]
struct Payload {
    target: String,
    price: Option<f64>,
}

#[derive(Debug)]
struct End {
    target: String,
    price: f64,
}

fn option_price(text: &str) -> Option<f64> {
    let json_value: Value = serde_json::from_str(&text).unwrap();
    let json_price = &json_value["result"]["price"];
    let option_price = json_price.as_f64();
    option_price
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let targets: Vec<String> = read_lines("targets.txt")?;
    let number_of_targets = targets.len();

    // https://docs.rs/futures-preview/0.3.0-alpha.17/futures/stream/trait.StreamExt.html#method.buffer_unordered
    // https://docs.rs/futures-preview/0.3.0-alpha.17/futures/stream/fn.iter.html
    let data = futures::stream::iter(
    targets.into_iter().map(|target| {
        async move {
            match reqwest::get(&target).await {
                Ok(body) => {
                    match body.text().await {
                        Ok(text) => {
                            // println!("From {}\n\n {}", &target, &text);
                            // let json_value: Value = serde_json::from_str(&text).unwrap();
                            // let json_price = &json_value["result"]["price"];
                            // let price = json_price.as_f64();
                            // let payload = Payload { target, price };

                            let price = option_price(&text);
                            let payload = Payload { target, price };
                            payload
                        }
                        Err(_) => {
                            println!("ERROR reading {}", &target);
                            let payload = Payload { target, price: None };
                            payload
                        }
                    }
                }
                Err(_) => {
                    println!("ERROR downloading {}", &target);
                    let payload = Payload { target, price: None };
                    payload
                }
            }
        }
    })
    ).buffer_unordered(number_of_targets).collect::<Vec<Payload>>();

    let payloads: Vec<Payload> = data.await;
    // Only extract market name here
    let ends: Vec<End> = payloads
        .into_iter()
        .filter(|payload| payload.price.is_some())
        .map(|end| End { target: end.target, price: end.price.unwrap() })
        .collect::<Vec<End>>();

    println!("{:#?}", &ends);
    Ok(())
}