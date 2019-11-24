#[derive(Deserialize, Serialize)]
pub struct TextElement {
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
pub struct ButtonElement {
  #[serde(rename = "type")]
  jsontype: String,
  text: TextElement,
  style: String,
  value: String,
}

#[derive(Deserialize, Serialize)]
pub enum Block {
  Section {
    jsontype: String,
    text: TextElement,
  },
  Actions {
    jsontype: String,
    elements: Vec<ButtonElement>,
  },
}

pub fn onboarding_blocks() -> Vec<Block> {
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

