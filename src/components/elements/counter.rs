use yew::prelude::*;

pub enum Msg {
    BtnClicked,
    IncTwo(u32),
    Reset,
}

pub struct Counter {
    pub count: u32,
}

impl Component for Counter {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <p>{"Counter"}</p>
                <p>{"Button was clicked "}{&self.count}{" times."}</p>
                <button onclick={ctx.link().callback(|_| Msg::BtnClicked)}>{"Click on me"}</button>
                <button onclick={ctx.link().callback(|_| Msg::IncTwo(2))}>{"Click to double increase"}</button>
                <button onclick={ctx.link().callback(|_| Msg::Reset)}>{"Click to reset"}</button>
            </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::BtnClicked => {
                self.count += 1;
                true // return true only if counter was updated
            }
            Msg::IncTwo(amount) => {
                self.count += amount;
                true
            }
            Msg::Reset => {
                self.count = 0;
                true
            }
        }
    }
}
