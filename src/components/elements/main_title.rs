use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::User;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub content: String,
    pub color: TextColor,
    pub on_load: Callback<String>,
}

#[derive(PartialEq)]
#[allow(unused)]
pub enum TextColor {
    Warn,
    Success,
    Normal,
}

impl TextColor {
    fn to_string(&self) -> String {
        match self {
            TextColor::Normal => "normal".into(),
            TextColor::Warn => "warn".into(),
            TextColor::Success => "success".into(),
        }
    }
}

const CSS: &str = include_str!("main_title.css");

#[styled_component(MainTitle)]
pub fn main_title(props: &Props) -> Html {
    let style = Style::new(CSS).unwrap();
    props.on_load.emit("Title loaded".into());
    let user_context = use_context::<User>().unwrap_or_default(); // USE CONTEXT
    html! {
        <div class={style}>
            <h1 class={&props.color.to_string()}>{&props.content}</h1>
            if user_context.age >= 18 {
                <p>{"Welcome, "} {user_context.name} {"!"}</p>
            } else {
                <p>{"You are too young.Your age is "} {user_context.age} {" so access denied"} </p>
            }

        </div>
    }
}
