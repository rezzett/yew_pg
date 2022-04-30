use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub ph: String,
    pub handle_onchange: Callback<String>,
    pub val: String,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let handle_onchange = props.handle_onchange.clone();
    let handler = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        handle_onchange.emit(value);
    });

    html! {
        <>
            <input  type="text"
                    name={props.name.clone()}
                    placeholder={props.ph.clone()}
                    onchange={handler}
                    value={props.val.clone()}
            />
            <br/>
        </>
    }
}
