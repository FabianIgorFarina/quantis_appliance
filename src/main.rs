use std::{collections::HashMap, sync::Arc};

use rand::prelude::*;
use rand_hc::Hc128Rng;
use std::sync::Mutex;
use warp::{hyper::StatusCode, Filter};

#[tokio::main]
async fn main() {
    let rng = Arc::new(Mutex::new(rand_hc::Hc128Rng::from_entropy()));

    let random_service = random_int_service(Arc::clone(&rng))
        .or(random_double_service(Arc::clone(&rng)))
        .or(random_short_service(Arc::clone(&rng)))
        .or(random_float_service(Arc::clone(&rng)));

    let cors = warp::cors().allow_any_origin();

    warp::serve(random_service)
        .tls()
        .cert_path("server1.crt")
        .key_path("server1.key")
        // .client_auth_required_path("ca.crt")
        .run(parse_address("127.0.0.1:8080"))
        .await;
}

fn parse_address(address: &str) -> ([u8; 4], u16) {
    let mut ip: [u8; 4] = [0; 4];
    let split: Vec<&str> = address.split(":").collect();
    let ip_split: Vec<&str> = split[0].split(".").collect();
    for i in 0..4 {
        ip[i] = ip_split[i].parse().unwrap();
    }
    (ip, split[1].parse::<u16>().unwrap())
}

fn random_int_service(
    rng: Arc<Mutex<Hc128Rng>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::path("api"))
        .and(warp::path("2.0"))
        .and(warp::path("int"))
        .and(warp::query().map(move |map: HashMap<String, i32>| {
            // default values
            let mut values: HashMap<String, i32> = HashMap::new();
            values.insert("min".to_string(), i32::MIN);
            values.insert("max".to_string(), i32::MAX);
            values.insert("quantity".to_string(), 1);
            for (key, value) in map.iter() {
                values.insert(key.clone(), *value);
            }
            if values.get("min") >= values.get("max") {
                return Ok(warp::reply::with_status(
                    String::from("Minimum can not be greater than maximum"),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            };
            let mut result: Vec<i32> = Vec::new();
            result.reserve(*values.get("quantity").unwrap() as usize);
            for _ in 0..(*values.get("quantity").unwrap() as usize) {
                result.push(
                    rng.lock()
                        .unwrap()
                        .gen_range((*values.get("min").unwrap())..(*values.get("max").unwrap())),
                );
            }
            Ok(warp::reply::with_status(
                format!("{:?}\n", result).to_string(),
                StatusCode::OK,
            ))
        }))
        .with(warp::cors().allow_any_origin())
}

fn random_short_service(
    rng: Arc<Mutex<Hc128Rng>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::path("api"))
        .and(warp::path("2.0"))
        .and(warp::path("int"))
        .and(warp::query().map(move |map: HashMap<String, i16>| {
            // default values
            let mut values: HashMap<String, i16> = HashMap::new();
            values.insert("min".to_string(), i16::MIN);
            values.insert("max".to_string(), i16::MAX);
            values.insert("quantity".to_string(), 1);
            for (key, value) in map.iter() {
                values.insert(key.clone(), *value);
            }
            if values.get("min") >= values.get("max") {
                return Ok(warp::reply::with_status(
                    String::from("Minimum can not be greater than maximum"),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            };
            let mut result: Vec<i16> = Vec::new();
            result.reserve(*values.get("quantity").unwrap() as usize);
            for _ in 0..(*values.get("quantity").unwrap() as usize) {
                result.push(
                    rng.lock()
                        .unwrap()
                        .gen_range((*values.get("min").unwrap())..(*values.get("max").unwrap())),
                );
            }
            Ok(warp::reply::with_status(
                format!("{:?}\n", result).to_string(),
                StatusCode::OK,
            ))
        }))
        .with(warp::cors().allow_any_origin())
}

fn random_double_service(
    rng: Arc<Mutex<Hc128Rng>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::path("api"))
        .and(warp::path("2.0"))
        .and(warp::path("double"))
        .and(warp::query().map(move |map: HashMap<String, f64>| {
            // default values
            let mut values: HashMap<String, f64> = HashMap::new();
            values.insert("min".to_string(), 0.0);
            values.insert("max".to_string(), 1.0);
            values.insert("quantity".to_string(), 1.0);
            for (key, value) in map.iter() {
                values.insert(key.clone(), *value);
            }
            if values.get("min") >= values.get("max") {
                return Ok(warp::reply::with_status(
                    String::from("Minimum can not be greater than maximum"),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            };
            let mut result: Vec<f64> = Vec::new();
            result.reserve(*values.get("quantity").unwrap() as usize);
            for _ in 0..(*values.get("quantity").unwrap() as usize) {
                result.push(
                    rng.lock()
                        .unwrap()
                        .gen_range((*values.get("min").unwrap())..(*values.get("max").unwrap())),
                );
            }
            Ok(warp::reply::with_status(
                format!("{:?}\n", result).to_string(),
                StatusCode::OK,
            ))
        }))
        .with(warp::cors().allow_any_origin())
}

fn random_float_service(
    rng: Arc<Mutex<Hc128Rng>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::path("api"))
        .and(warp::path("2.0"))
        .and(warp::path("float"))
        .and(warp::query().map(move |map: HashMap<String, f32>| {
            // default values
            let mut values: HashMap<String, f32> = HashMap::new();
            values.insert("min".to_string(), 0.0);
            values.insert("max".to_string(), 1.0);
            values.insert("quantity".to_string(), 1.0);
            for (key, value) in map.iter() {
                values.insert(key.clone(), *value);
            }
            if values.get("min") >= values.get("max") {
                return Ok(warp::reply::with_status(
                    String::from("Minimum can not be greater than maximum"),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            };
            let mut result: Vec<f32> = Vec::new();
            result.reserve(*values.get("quantity").unwrap() as usize);
            for _ in 0..(*values.get("quantity").unwrap() as usize) {
                result.push(
                    rng.lock()
                        .unwrap()
                        .gen_range((*values.get("min").unwrap())..(*values.get("max").unwrap())),
                );
            }
            Ok(warp::reply::with_status(
                format!("{:?}/n", result).to_string(),
                StatusCode::OK,
            ))
        }))
        .with(warp::cors().allow_any_origin())
}
