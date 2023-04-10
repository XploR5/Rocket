#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use chrono::{DateTime, Utc};
use fastrand::i32;
use postgres::{Client, NoTls};
use rand;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    plant_id: i32,
    createdat: String,
    quality: i32,
    performance: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateDataRequest {
    plant_id: Vec<i32>,
    start_date: String,
    end_date: String,
    interval: i64,
}

#[post("/createdata", format = "json", data = "<request>")]
fn create_data(request: Json<CreateDataRequest>) -> Json<HashMap<&'static str, &'static str>> {
    let mut client =
        Client::connect("postgres://postgres:password@localhost/testing_db", NoTls).unwrap();

    let mut list = vec![]; // Data is stored here before inserting into the db
                           // Data is created here and stored in a vector
    for i in 0..request.plant_id.len() {
        let mut current_date = chrono::DateTime::parse_from_rfc3339(&request.start_date)
            .unwrap()
            .with_timezone(&chrono::Utc);
        let end_date = chrono::DateTime::parse_from_rfc3339(&request.end_date)
            .unwrap()
            .with_timezone(&chrono::Utc);

        while current_date <= end_date {
            let obj = (
                request.plant_id[i],
                current_date.to_rfc3339(), // convert to string
                rand::random::<i32>() % 100 + 1,
                rand::random::<i32>() % 100 + 1,
            );
            list.push(obj);
            current_date = current_date + chrono::Duration::seconds(request.interval as i64);
        }
    }

    // Data insertion into the db starts here
    for row in &list {
        client.execute(
            "INSERT INTO single (plant_id, createdat, quality, performance) VALUES ($1, $2, $3, $4)",
            &[&row.0, &row.1, &row.2, &row.3]
        ).unwrap();
    }
    let mut map = HashMap::new();
    map.insert("status", "success");
    Json(map)
}

fn main() {
    rocket::ignite().mount("/", routes![create_data]).launch();
}
