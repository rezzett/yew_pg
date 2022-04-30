use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::pages::{
    about::About, contacts::Contacts, counter_page::CounterPage, faq::Faq, home::Home,
    not_found::NotFound,
};

#[derive(Routable, Copy, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/contacts")]
    Contacts,
    #[at("/faq")]
    Faq,
    #[at("/counter")]
    Counter,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {<Home/>},
        Route::About => html! {<About/>},
        Route::Contacts => html! {<Contacts/>},
        Route::Faq => html! {<Faq content={"Just some question".to_string()}/>},
        Route::Counter => html! {<CounterPage/>},
        Route::NotFound => html! {<NotFound/>},
    }
}
