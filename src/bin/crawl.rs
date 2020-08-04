extern crate bodegas;
extern crate diesel;
extern crate reqwest;

use rayon::prelude::*;
use std::collections::HashMap;

use self::bodegas::*;
use self::diesel::prelude::*;
use self::models::*;
use schema::stores;

#[derive(Debug)]
struct Coords {
    north: f64,
    east: f64,
    south: f64,
    west: f64,
}

const COUNTRY_CODE: &str = "PE";

fn build_body(
    south: f64,
    west: f64,
    north: f64,
    east: f64,
    zoom: i32,
    country_code: &String,
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
    area_coords: &Coords,
) -> Option<HashMap<i32, Store>> {
    let Coords {
        north,
        east,
        south,
        west,
    } = area_coords;

    let zoom = 16;
    let country_code = COUNTRY_CODE.to_lowercase();

    let body = build_body(*south, *west, *north, *east, zoom, &country_code);
    let endpoint = format!(
        "https://1fzqk3npw4.execute-api.us-east-1.amazonaws.com/nearby_store_stage/{}",
        country_code
    );
    let response: reqwest::blocking::Response = client.post(&endpoint).body(body).send().unwrap();

    // response.json().expect("Some error:")
    return match response.json() {
        Ok(hash) => Some(hash),
        Err(error) => {
            println!("[❗ {}] Coords {:?}", error, area_coords);
            None
        }
    };
}

fn store_response(conn: &PgConnection, result: HashMap<i32, Store>) {
    let mut successful_inserts = 0;

    for store in result.values() {
        let insert_result = diesel::insert_into(stores::table)
            .values(store)
            .get_result::<Store>(conn);

        match insert_result {
            Ok(_) => successful_inserts += 1,
            Err(_) => println!("[❗] Error while saving store: {:?}", store),
        }
    }

    if successful_inserts > 0 {
        println!("[✔] Inserted {} stores", successful_inserts);
    }
}

fn divide_area(area_coords: &Coords) -> Vec<Coords> {
    let Coords {
        north,
        east,
        south,
        west,
    } = area_coords;

    // Area Calc
    let side_diff = 0.012;
    let mut coords_arr: Vec<Coords> = Vec::new();

    let mut south_pivot = *south;
    while south_pivot < *north {
        let mut north_pivot = south_pivot + side_diff;
        if north_pivot > *north {
            north_pivot = *north;
        }

        let mut west_pivot = *west;
        while west_pivot < *east {
            let mut east_pivot = west_pivot + side_diff;
            if east_pivot > *east {
                east_pivot = *east;
            }

            coords_arr.push(Coords {
                north: north_pivot,
                east: east_pivot,
                south: south_pivot,
                west: west_pivot,
            });

            west_pivot = east_pivot;
        }

        south_pivot = north_pivot;
    }

    coords_arr
}

fn process_areas_in_parallel(areas: Vec<Coords>) {
    let areas_len = areas.len();
    let thread_load = areas_len / 64;
    let step = if thread_load > 1000 {
        1000
    } else {
        thread_load
    };

    let mut slices = Vec::new();

    for slice_start in (0..areas_len).step_by(step) {
        let slice_end = if slice_start + step > areas_len {
            areas_len
        } else {
            slice_start + step
        };

        slices.push(&areas[slice_start..slice_end]);
    }

    slices.par_iter().for_each(|areas_slice| {
        // Reqwest client
        let client = reqwest::blocking::Client::new();
        // Diesel client
        let connection = establish_connection();

        // Requests and DB insertion
        for coords in *areas_slice {
            if let Some(stores) = request_stores(&client, coords) {
                store_response(&connection, stores);
            }
        }
    });
}

fn main() {
    // Peru coords
    let country_areas: Vec<Coords> = vec![
        Coords {
            north: -11.297756,
            east: -68.758453,
            south: -18.439670,
            west: -77.668796,
        },
        Coords {
            north: 0.005921,
            east: -69.149799,
            south: -11.297756,
            west: -81.322651,
        },
    ];

    for (_i, area) in country_areas.iter().enumerate() {
        let areas = divide_area(&area);
        // , &format!("{}{}", COUNTRY_CODE, i)
        let areas_to_request = areas.len();
        println!("Areas to request: {}", areas_to_request);

        process_areas_in_parallel(areas);
    }
}
