use stylist::Style;
use yew::prelude::*;

pub struct Faq {
    pub msg: String,
    css: Style,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub content: Option<String>,
}

const CSS: &str = include_str!("faq.css");

impl Component for Faq {
    type Message = ();

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            msg: "Frequently asked questions".into(),
            css: Style::new(CSS).unwrap(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <span class={self.css.clone()}>
                    <h4 >{&self.msg}</h4>
                </span>
                <p>{ctx.props().content.clone().unwrap()}</p>
            </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}
