#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;


mod post_model;
use post_model::Resp;

use serde::Serialize;
use serde::Deserialize;
use rocket_contrib::json::Json;
use std::intrinsics::copy;
use std::thread;

mod resize_image;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[post("/post", data = "<name>")]
fn n(name: Json<Vec<Resp>>)-> String {
    for e in name.iter() {
        println!("{:?}", e);
        resize_image::resize_image_from_url(&e.url);

    }
    String::from(&name[0].url)
}

fn main() {
    rocket::ignite().mount("/", routes![index, n]).launch();
}