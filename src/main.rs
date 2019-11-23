#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use rocket_contrib::json::Json;
use serde_json::{json, Result, Value};

#[derive(Deserialize, Serialize)]
struct TextElement {
  #[serde(rename = "type")]
  jsontype: String,
  #[serde(default = "default_text_element_emoji")]
  emoji: bool,
  text: String,
}

fn default_text_element_emoji() -> bool {
  true
}

#[derive(Deserialize, Serialize)]
struct ButtonElement {
  #[serde(rename = "type")]
  jsontype: String,
  text: TextElement,
  style: String,
  value: String,
}

#[derive(Deserialize, Serialize)]
enum Block {
  Section {
    jsontype: String,
    text: TextElement,
  },
  Actions {
    jsontype: String,
    elements: Vec<ButtonElement>,
  },
}

fn onboarding_blocks() -> Vec<Block> {
  vec![
    Block::Section {
      jsontype: "section".to_owned(),
      text: TextElement {
        jsontype: "mrkdwn".to_owned(),
        emoji: false,
        text: "Congratulations, you're part of the breakfast crew this week! \
          Here's what you need to do:
          *Tuesday*: groceries delivery day, put them away
          *Wednesday*: prepare breakfast, clean up after breakfast".to_owned(),
      },
    },
    Block::Section {
      jsontype: "section".to_owned(),
      text: TextElement {
        jsontype: "mrkdwn".to_owned(),
        emoji: false,
        text: "Please indicate your availability by choosing one of the options below.".to_owned(),
      },
    },
    Block::Actions {
      jsontype: "actions".to_owned(),
      elements: vec![
        ButtonElement {
          jsontype: "button".to_owned(),
          text: TextElement {
            jsontype: "plain_text".to_owned(),
            emoji: true,
            text: ":+1: I am available".to_owned(),
          },
          style: "primary".to_owned(),
          value: "breakfast_ok_123".to_owned(),
        },
        ButtonElement {
          jsontype: "button".to_owned(),
          text: TextElement {
            jsontype: "plain_text".to_owned(),
            emoji: true,
            text: ":-1: I'm not available".to_owned(),
          },
          style: "danger".to_owned(),
          value: "breakfast_not_ok_123".to_owned(),
        }
      ],
    }
  ]
}

#[get("/")]
fn index() -> Json<Vec<Block>> {
  Json(onboarding_blocks())
}

fn main() {
  rocket::ignite().mount("/", routes![index]).launch();
}
