extern crate bodegas;
extern crate diesel;
extern crate reqwest;

use std::collections::HashMap;

use self::bodegas::*;
use self::diesel::prelude::*;
use self::models::*;
use schema::stores;

fn build_body(
    south: f64,
    west: f64,
    north: f64,
    east: f64,
    zoom: i32,
    country_code: &str,
) -> String {
    format!("{{\"south\":{south},\"west\":{west},\"north\":{north},\"east\":{east},\"zoom\":{zoom},\"country_code\":\"{country_code}\"}}",
    south = south,
    west = west,
    north = north,
    east = east,
    zoom = zoom,
    country_code= country_code)
}

fn request_stores(
    client: &reqwest::blocking::Client,
    south: f64,
    west: f64,
    north: f64,
    east: f64,
) -> HashMap<i32, Store> {
    let zoom = 20;
    let country_code = "pe";

    let body = build_body(south, west, north, east, zoom, country_code);
    let response: reqwest::blocking::Response = client
        .post("https://1fzqk3npw4.execute-api.us-east-1.amazonaws.com/nearby_store_stage/pe")
        .body(body)
        .send()
        .unwrap();
    response.json().expect("Some error:")
}

fn store_response(conn: &PgConnection, result: HashMap<i32, Store>) {
    for store in result.values() {
        let insert_result = diesel::insert_into(stores::table)
            .values(store)
            .get_result::<Store>(conn);

        match insert_result {
            Ok(_) => (),
            Err(_) => println!("[!] Error while saving store: {:?}", store),
        }
    }
}

fn main() {
    // Reqwest client
    let client = reqwest::blocking::Client::new();
    // Diesel client
    let connection = establish_connection();

    let south = -12.003392325421665;
    let west = -77.09370711281127;
    let north = -11.982277061323614;
    let east = -77.06370928718871;

    // let south = -12.233602;
    // let west = -77.186818;
    // let north = -11.857807;
    // let east = -76.825380;
    let result = request_stores(&client, south, west, north, east);
    store_response(&connection, result);
}
