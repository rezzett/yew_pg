use stylist::{yew::styled_component, Style};
use yew::prelude::*;

static CSS: &str = include_str!("button.css");

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: String,
    pub bg: ButtonBg,
    pub shape: BtnShape,
    pub onclick_handler: Callback<()>,
}

#[allow(unused)]
#[derive(PartialEq)]
pub enum ButtonBg {
    Success,
    Warn,
    Normal,
    Error,
}

impl ButtonBg {
    fn to_string(&self) -> String {
        match self {
            ButtonBg::Success => "success".into(),
            ButtonBg::Normal => "normal".into(),
            ButtonBg::Warn => "warn".into(),
            ButtonBg::Error => "error".into(),
        }
    }
}

#[derive(PartialEq)]
#[allow(unused)]
pub enum BtnShape {
    Sharp,
    Round,
    Circle,
}

impl BtnShape {
    fn to_string(&self) -> String {
        match self {
            BtnShape::Sharp => "sharp".into(),
            BtnShape::Round => "round".into(),
            BtnShape::Circle => "circle".into(),
        }
    }
}

#[styled_component(Button)]
pub fn button(props: &Props) -> Html {
    let style = Style::new(CSS).unwrap();
    let onclick_handler = props.onclick_handler.clone();
    let handler = Callback::from(move |_| onclick_handler.emit(()));
    html! {
        <div class= {style}>
            <button type="button" onclick={handler}  class={classes!(props.bg.to_string(), &props.shape.to_string())}>{&props.text}</button>
        </div>
    }
}
