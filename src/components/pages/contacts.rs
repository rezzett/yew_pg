use crate::{
    components::elements::adder::Adder,
    components::elements::contact_form::ContactForm,
    store::{init, YewduxStore},
};
use yew::prelude::*;
use yewdux::prelude::{BasicStore, Dispatch, DispatchProps, WithDispatch};

pub struct Contacts {
    _dispatch: Dispatch<BasicStore<YewduxStore>>,
}

impl Component for Contacts {
    type Message = ();

    type Properties = DispatchProps<BasicStore<YewduxStore>>;

    fn create(_ctx: &Context<Self>) -> Self {
        let _dispatch = init();
        Self { _dispatch }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h4>{"Contacts"}</h4>
                <Adder />
                <WithDispatch<ContactForm> />
            </>
        }
    }
}
