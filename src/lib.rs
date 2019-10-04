#![recursion_limit = "512"]

mod device;
mod device_modal;
mod devices;

use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

use crate::devices::Devices;

pub struct RootModel;

impl Component for RootModel {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        RootModel {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<RootModel> for RootModel {
    fn view(&self) -> Html<Self> {
        html! {
            <section class="section">
                <div class="continer">
                <h2 class="title">
            {"My nodes"}
            </h2>
                <Devices />
                f</div>
                </section>
        }
    }
}
