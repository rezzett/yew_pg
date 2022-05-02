use yew::prelude::*;
use yewdux::prelude::{BasicStore, Dispatcher};
use yewdux_functional::use_store;

use crate::store::YewduxStore;

#[function_component(Adder)]
pub fn adder() -> Html {
    let store = use_store::<BasicStore<YewduxStore>>();
    let add_handler = store
        .dispatch()
        .reduce_callback(|state| state.adder_val += 1);

    let sub_handler = store
        .dispatch()
        .reduce_callback(|state| state.adder_val -= 1);

    let adder_val = store.state().map(|s| s.adder_val).unwrap_or_default();

    html! {
        <>
            <p>{adder_val}</p>
            <button onclick={add_handler}>{"+"}</button>
            <button onclick={sub_handler}>{"-"}</button>
        </>
    }
}
