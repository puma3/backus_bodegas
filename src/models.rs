use serde::Deserialize;

use super::schema::stores;

#[derive(Queryable, Deserialize, Debug, Insertable, Clone)]
#[table_name = "stores"]
pub struct Store {
    pub id: i32,
    pub title: Option<String>,
    pub lat: f64,
    pub lng: f64,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub country_code: Option<String>,
    pub icon: Option<String>,
    pub code: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub delivery: Option<i32>,
    pub store_url: Option<String>,
    pub promo1: Option<String>,
    pub promo2: Option<String>,
    pub promo3: Option<String>,
    pub data: Option<String>,
}
