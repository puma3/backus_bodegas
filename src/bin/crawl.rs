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
    let zoom = 16;
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
    let mut successful_inserts = 0;

    for store in result.values() {
        let insert_result = diesel::insert_into(stores::table)
            .values(store)
            .get_result::<Store>(conn);

        match insert_result {
            Ok(_) => successful_inserts += 1,
            Err(_) => println!("[!] Error while saving store: {:?}", store),
        }
    }

    print!("Inserted {} stores", successful_inserts);
}

fn iterate_through_area(north: f64, east: f64, south: f64, west: f64, area: &str) {
    // Area Calc
    let side_diff = 0.01575;
    let mut coords_arr: Vec<(f64, f64, f64, f64)> = Vec::new();

    let mut north_pivot = north;

    while north_pivot < south {
        let mut south_pivot = north_pivot + side_diff;
        if south_pivot > south {
            south_pivot = south;
        }
        let mut east_pivot = east;

        while east_pivot < west {
            let mut west_pivot = east_pivot + side_diff;
            if west_pivot > west {
                west_pivot = west;
            }

            coords_arr.push((north_pivot, east_pivot, south_pivot, west_pivot));
            east_pivot = west_pivot;
        }
        north_pivot = south_pivot;
    }

    let areas_to_request = coords_arr.len();
    println!("Areas to request: {}", areas_to_request);

    // Reqwest client
    let client = reqwest::blocking::Client::new();
    // Diesel client
    let connection = establish_connection();
    // Requests and DB insertion
    let mut n_request = 0;
    for (north, east, south, west) in coords_arr {
        n_request += 1;
        let result = request_stores(&client, north, east, south, west);
        print!("[{}/{} | {}]\t", n_request, areas_to_request, area);
        store_response(&connection, result);
        println!("\t({}, {}, {}, {})", north, east, south, west);
    }
}

fn main() {
    // Reference
    // let south = -12.07778082305759;
    // let west = -76.9738632921875;
    // let north = -12.062034945616542;
    // let east = -76.9409043078125;

    // let south = -12.003392325421665;
    // let west = -77.09370711281127;
    // let north = -11.982277061323614;
    // let east = -77.06370928718871;

    // Lima
    // let north = -12.2295735;
    // let east = -77.1391195;
    // let south = -11.947827;
    // let west = -76.8175587;

    // Peru
    // let north = -18.2886053;
    // let east = -82.6732716;
    // let south = 0.7666798;
    // let west = -68.3295077;

    // South 1
    // let north = -17.188226;
    // let east = -72.572151;
    // let south = -16.112174;
    // let west = -68.717678;

    // South 2
    // let north = -17.399415;
    // let east = -75.171298;
    // let south = -17.188226;
    // let west = -72.572151;

    // Center 1
    // let north = -17.188226;
    // let east = -76.443410;
    // let south = -13.163580;
    // let west = -68.498418;

    // Center 2
    // let north = -13.163580;
    // let east = -78.319916;
    // let south = -9.751493;
    // let west = -68.498418;

    // North 1
    // let north = -9.751493;
    // let east = -79.750866;
    // let south = -7.190257;
    // let west = -71.983532;

    // North A
    // let north = -7.190257;
    // let east = -81.673473;
    // let south = -3.000069;
    // let west = -76.685680;

    // North B failed
    // let north = -7.190257;
    // let east = -76.685680;
    // let south = 0.030786;
    // let west = -69.720348;

    // North B1
    // let north = -6.198007000000018;
    // let east = -76.685680;
    // let south = -3.1143965;
    // let west = -73.203014;

    // North B2
    // let north = -3.1143965;
    // let east = -73.203014;
    // let south = 0.030786;
    // let west = -69.720348;

    // North B3
    // let north = -6.198007000000018;
    // let east = -76.685680;
    // let south = -3.1143965;
    // let west = -73.203014;

    // North B4
    // let north = -3.1143965;
    // let east = -73.203014;
    // let south = 0.030786;
    // let west = -69.720348;

    // Center 2A
    // let north = -10.218329999999888;
    // let east = -78.319916;
    // let south = -9.751493;
    // let west = -77.09223;

    // Center 2B
    let north = -10.218329999999888;
    let east = -77.09223;
    let south = -9.751493;
    let west = -75.86454;

    // // Center 2C
    // let north = -10.218329999999888;
    // let east = -75.86454;
    // let south = -9.751493;
    // let west = -74.636856;

    // // Center 2D
    // let north = -10.218329999999888;
    // let east = -74.636856;
    // let south = -9.751493;
    // let west = -73.409164;

    // // Center 2E
    // let north = -10.218329999999888;
    // let east = -73.409164;
    // let south = -9.751493;
    // let west = -72.18148;

    // // Center 2F
    // let north = -10.218329999999888;
    // let east = -72.18148;
    // let south = -9.751493;
    // let west = -70.9538;

    // // Center 2G
    // let north = -10.218329999999888;
    // let east = -70.9538;
    // let south = -9.751493;
    // let west = -69.726105;

    // // Center 2H
    // let north = -10.218329999999888;
    // let east = -69.726105;
    // let south = -9.751493;
    // let west = -68.498418;

    iterate_through_area(north, east, south, west, "C2B");
}
