use yewdux::prelude::{BasicStore, Dispatch};

#[derive(Clone, Default)]
pub struct YewduxStore {
    pub adder_val: i32,
    pub contact_name: String,
}

pub fn init() -> Dispatch<BasicStore<YewduxStore>> {
    Dispatch::<BasicStore<YewduxStore>>::new()
}
