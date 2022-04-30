use std::ops::Deref;

use crate::components::elements::{
    button::{BtnShape, Button, ButtonBg},
    text_input::TextInput,
};
use gloo::console::log;
use serde::Serialize;
use yew::prelude::*;

#[derive(Default, Clone, Serialize)]
pub struct FormState {
    pub login: String,
    pub pwd: String,
    pub age: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit_handler: Callback<FormState>,
}

#[function_component(LoginForm)]
pub fn login_form(props: &Props) -> Html {
    let state = use_state(|| FormState::default());

    let cloned_state = state.clone();
    let login_changed = Callback::from(move |login| {
        cloned_state.set(FormState {
            login,
            ..cloned_state.deref().clone()
        });
    });

    let cloned_state = state.clone();
    let password_changed = Callback::from(move |pwd| {
        cloned_state.set(FormState {
            pwd,
            ..cloned_state.deref().clone()
        });
    });

    let cloned_state = state.clone();
    let age_changed = Callback::from(move |age| {
        cloned_state.set(FormState {
            age,
            ..cloned_state.deref().clone()
        });
    });

    let cloned_state = state.clone();
    let on_submit = props.onsubmit_handler.clone();
    let btn_onclick = Callback::from(move |_| {
        log!("Form sended");
        on_submit.emit(cloned_state.deref().clone());
        cloned_state.set(FormState::default());
    });

    html! {
        <>
            <form>
                <TextInput name="login" ph="Login" handle_onchange={login_changed} val={state.login.clone()}/>
                <TextInput name="password" ph="Password" handle_onchange={password_changed}  val={state.pwd.clone()}/>
                <TextInput name="age" ph="Age" handle_onchange={age_changed} val={state.age.clone()}/>
                <Button text="Sign In" bg={ButtonBg::Normal} shape={BtnShape::Round} onclick_handler={btn_onclick} />
            </form>
            <div>
                <p>{"Login: "}{&state.login}</p>
                <p>{"Password: "}{&state.pwd}</p>
            </div>
        </>
    }
}
