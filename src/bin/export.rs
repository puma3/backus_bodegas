extern crate bodegas;
extern crate csv;
extern crate diesel;

use self::bodegas::*;
use self::diesel::prelude::*;
use self::models::*;

fn get_stores(limit: i64, offset: i64) -> Vec<Store> {
    use bodegas::schema::stores::dsl::*;

    let connection = establish_connection();
    stores
        .limit(limit)
        .offset(offset)
        .load::<Store>(&connection)
        .expect("Error loading posts")
}

fn save_stores(stores: &Vec<Store>, batch_no: i64) {
    let file_name = format!("output/batch_{}.csv", batch_no);
    let mut wtr = csv::Writer::from_path(file_name).unwrap();

    wtr.write_record(&[
        "N�mero",
        "Nombres",
        "Apellidos",
        "Dato 1",
        "Dato 2",
        "Dato 3",
        "Dato 4",
        "Dato 5",
        "Dato 6",
        "Dato 7",
        "Dato 8",
        "Dato 9",
        "Dato10",
    ])
    .expect("Error while writing header");

    for store in stores {
        let phone = store.phone.clone();
        let phone_str = phone.unwrap();

        if phone_str.len() > 0 {
            wtr.write_record(&[&phone_str, "", "", "", "", "", "", "", "", "", "", "", ""])
                .expect("Error while writing header");
        }
    }
}

fn main() {
    let records_count = 113505;
    let page_size = 20000;

    let pages_count = ((records_count as f32) / (page_size as f32)).ceil() as i64;

    for page in 0..pages_count {
        let offset = page * page_size;
        let results = get_stores(page_size, offset);
        save_stores(&results, page + 1);
    }
}
