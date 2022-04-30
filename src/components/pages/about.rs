use crate::{
    components::elements::button::{BtnShape, Button, ButtonBg},
    router::Route,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    let history = use_history().unwrap();
    let cb = Callback::from(move |_| history.push(Route::Home));
    html! {
        <>
            <h4>{"About"}</h4>
            <Button text={"Home"}  bg={ButtonBg::Normal} shape={BtnShape::Round} onclick_handler={cb} />
        </>
    }
}
