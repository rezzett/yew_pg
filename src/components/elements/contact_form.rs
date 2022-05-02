use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, MouseEvent};
use yew::{html, Callback, Component};
use yewdux::prelude::{BasicStore, DispatchProps, Dispatcher, WithDispatchProps};

use crate::store::YewduxStore;

pub struct ContactForm {
    _dispatch: DispatchProps<BasicStore<YewduxStore>>,
}

impl Component for ContactForm {
    type Message = ();

    type Properties = DispatchProps<BasicStore<YewduxStore>>;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            _dispatch: ctx.props().dispatch().clone(),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let contact_name_handler =
            ctx.props()
                .dispatch()
                .reduce_callback_with(|state, e: Event| {
                    let val = e
                        .target()
                        .unwrap()
                        .unchecked_into::<HtmlInputElement>()
                        .value();
                    state.contact_name = val;
                });

        let state = ctx.props().state().clone();
        let submit_btn_handler =
            Callback::from(move |_: MouseEvent| log!(state.contact_name.clone()));

        html! {
            <div>
                <form >
                    <label for="contact_name">{"Contact name"}</label><br/>
                    <input type="text" id="contact_name" onchange={contact_name_handler} />
                    <button onclick={submit_btn_handler} type="button">{"Add Contact"}</button>
                </form>
            </div>
        }
    }
}
