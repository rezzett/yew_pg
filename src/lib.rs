use std::ops::Deref;
use std::vec;

use gloo::console::log;
use serde::{Deserialize, Serialize};
use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yew::ContextProvider;
use yew_router::prelude::*;

mod components;
mod router;
use components::blocks::login_form::LoginForm;
use components::elements::main_title::{MainTitle, TextColor};
mod store;

use crate::components::blocks::login_form::FormState;
use crate::router::{switch, Route};

#[derive(Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct User {
    pub name: String,
    pub pwd: String,
    pub age: u8,
}

const APP_CSS: &str = include_str!("app.css");

#[styled_component(App)]
pub fn app() -> Html {
    let user_state = use_state(User::default);
    let is_first_render = use_state(|| true);
    let menu_items = vec![
        (Route::Home, "Home"),
        (Route::About, "About"),
        (Route::Contacts, "Contacts"),
        (Route::Faq, "FAQ"),
        (Route::Counter, "Counter"),
    ];
    log!(serde_json::to_string_pretty(&*user_state)
        .expect("Cannot convert User to strin on log macro"));
    let app_style = Style::new(APP_CSS).unwrap();
    let main_title_onload = Callback::from(|msg: String| log!(msg));

    let cloned_state = user_state.clone();
    let form_submit_handler = Callback::from(move |data: FormState| {
        log!(serde_json::to_string(&data).unwrap());
        cloned_state.set(User {
            name: data.login.clone(),
            pwd: data.pwd.clone(),
            age: data.age.parse::<u8>().unwrap_or(0),
        })
    });

    use_effect(move || {
        // this code will run on first render and rerender
        // check if auth token exists
        // chek if it is first render (to avoid infinity call use_effect)
        if *is_first_render {
            // this code will run only on first render
            is_first_render.set(false);
        }

        // closure to clean up
        || {}
    });

    html! {
        <ContextProvider<User> context={user_state.deref().clone()}>
            <MainTitle content="MAIN TITLE" color={TextColor::Warn} on_load={main_title_onload}/>


            <p class={css!("color: red;")}>{"lorem ipusm"}</p>

            <LoginForm onsubmit_handler={form_submit_handler}/>
            <BrowserRouter>
                <ul class={app_style}>
                    {menu_items.iter().map(|i| html!{<li><Link<Route> to={i.0}>{i.1}</Link<Route>></li>}).collect::<Html>()}
                </ul>
                <Switch<Route> render={Switch::render(switch)}/>
            </BrowserRouter>
        </ContextProvider<User>>
    }
}
