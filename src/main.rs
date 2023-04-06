#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use chrono::{DateTime, Utc};
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

#[derive(Debug, Serialize, Deserialize)]
struct CreateDataRequest {
    plant_id: Vec<i32>,
    start_date: String,
    end_date: String,
    interval: i64,
}

#[post("/createdata")]
fn _create_data() -> Json<HashMap<&'static str, &'static str>> {
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

#[post("/createdata", format = "json", data = "<request>")]
fn create_data(request: Json<CreateDataRequest>) -> Json<HashMap<&'static str, &'static str>> {
    let client = Client::connect("postgres://postgres:password@localhost/testing_db", NoTls).unwrap();

    for plant in request.plant_id.iter() {
        let start_datetime = DateTime::parse_from_rfc3339(&request.start_date)
            .unwrap()
            .with_timezone(&Utc);
        let end_datetime = DateTime::parse_from_rfc3339(&request.end_date)
            .unwrap()
            .with_timezone(&Utc);
        let interval_duration = chrono::Duration::minutes(request.interval);

        print!("plant: {}, st_dt: {}, end_dt: {}, int: {} \n\n",*plant, start_datetime, end_datetime, interval_duration )

        // create_and_insert_data(*plant, start_datetime, end_datetime, interval_duration, &client)
        //     .await
        //     .unwrap();
    }

    let mut map = HashMap::new();
    map.insert("status", "success");
    Json(map)
}



fn main() {
    rocket::ignite().mount("/", routes![create_data]).launch();
}