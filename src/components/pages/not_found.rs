use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <>
            <h1>{"404"}</h1>
            <p>{"Page not found :-("}</p>
            <Link<Route> to={Route::Home} >{"Home"}</Link<Route>>
        </>
    }
}
