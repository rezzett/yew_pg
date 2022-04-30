use gloo::console::log;
use yew::prelude::*;
use yewdux::prelude::{BasicStore, DispatchProps, Dispatcher, WithDispatchProps};

use crate::store::YewduxStore;

pub struct Adder {
    _dispatch: DispatchProps<BasicStore<YewduxStore>>, // store it fo force rerender when state updates
}

impl Component for Adder {
    type Message = ();

    type Properties = DispatchProps<BasicStore<YewduxStore>>;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            _dispatch: ctx.props().dispatch().clone(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log!("render"); // TODO form and counter rerender together when one of those changed
        let adder_val = ctx.props().state().adder_val; // struct YeduxStore under Rc

        let add_handler = ctx
            .props()
            .dispatch()
            .reduce_callback(|state| state.adder_val += 1);

        let sub_handler = ctx
            .props()
            .dispatch()
            .reduce_callback(|state| state.adder_val -= 1);

        html! {
            <>
                <p>{adder_val}</p>
                <button onclick={add_handler}>{"+"}</button>
                <button onclick={sub_handler}>{"-"}</button>
            </>
        }
    }
}
