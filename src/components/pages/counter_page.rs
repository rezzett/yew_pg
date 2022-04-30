use yew::prelude::*;

use crate::components::elements::counter::Counter;

#[function_component(CounterPage)]
pub fn counter_page() -> Html {
    html! {
        <>
            <h4>{"Counter Page"}</h4>
            <Counter/>
        </>
    }
}
