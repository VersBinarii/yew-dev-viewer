#![recursion_limit = "512"]

mod device;
mod devices;
mod device_modal;

use stdweb::web::Date;
use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

use crate::devices::Devices;

pub struct RootModel {
  console: ConsoleService,
}

impl Component for RootModel {
  type Message = ();
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    RootModel {
      console: ConsoleService::new(),
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    true
  }
}

impl Renderable<RootModel> for RootModel {
  fn view(&self) -> Html<Self> {
    html! {
        <section class="section">
            <div class="continer">
            <h2 class="title">
              {"Fjord Telecom nodes"}
            </h2>
            <Devices />
            </div>
        </section>
    }
  }
}
