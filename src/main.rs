#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::json::Json;

use blocks::Block;

mod blocks;

#[get("/")]
fn index() -> Json<Vec<Block>> {
  Json(blocks::onboarding_blocks())
}

fn main() {
  rocket::ignite().mount("/", routes![index]).launch();
}
