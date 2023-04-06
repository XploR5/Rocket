#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket_contrib::json::Json;
use std::collections::HashMap;
use postgres::{Client, NoTls};
use fastrand::i32;
use serde::{Serialize, Deserialize};

#[derive(Debug,  Serialize, Deserialize)]
struct Data {
    plant_id: i32,
    createdat: String,
    quality: i32,
    performance: i32
}

#[post("/createdata")]
fn create_data() -> Json<HashMap<&'static str, &'static str>> {
    let mut client = Client::connect("postgres://postgres:password@localhost/testing_db", NoTls).unwrap();

    for _ in 0..10 {
        let plant_id =  i32(..=10);
        let createdat = chrono::Utc::now().to_rfc3339();
        let quality =  i32(..=100);
        let performance =  i32(..=100);

        client.execute(
            "INSERT INTO single (plant_id, createdat, quality, performance) VALUES ($1, $2, $3, $4)",
            &[&plant_id, &createdat, &quality, &performance]
        ).unwrap();
    }

    let mut map = HashMap::new();
    map.insert("status", "success");
    Json(map)
}

fn main() {
    rocket::ignite().mount("/", routes![create_data]).launch();
}