use crate::device::{Device, Interface};
use failure::Error;
use stdweb::unstable::TryInto;
use stdweb::web::{event::IEvent, Element, FormData};
use yew::format::Json;
use yew::prelude::*;
use yew::services::{
    fetch::{FetchService, FetchTask, Request, Response},
    ConsoleService,
};
use yew::{Component, ComponentLink, Html, Renderable, ShouldRender};

pub enum DeviceModalMsg {
    HideModal,
    EditDevice,
    FinishEdit,
    SubmitDevice(FormData),
    SubmitSuccess,
    SubmitFail,
}

#[derive(Properties)]
pub struct DeviceModalProps {
    #[props(required)]
    pub device: Device,
    #[props(required)]
    pub visible: bool,
    #[props(required)]
    pub on_close: Callback<bool>,
}

pub struct DeviceModal {
    pub device: Device,
    pub visible: bool,
    pub on_close: Callback<bool>,
    is_editing: bool,
    fetch: FetchService,
    task: Option<FetchTask>,
    link: ComponentLink<DeviceModal>,
}

impl DeviceModal {
    fn render_interfaces(&self, i: &Interface) -> Html<Self> {
        if self.is_editing {
            html! {
                <div class="columns">
                  <div class="column">
                    <p>
                      <strong>{"IP: "}</strong>
                      <input class="input" name="iface-address"
                               value=&i.interface/>
                    </p>
                  </div>
                  <div class="column">
                    <p><strong>{"Check type: "}</strong>
                    <input class="input" name="iface-check-method" value=&i.check_method/>
                    </p>
                  </div>
                  <div class="column">
                    <p><strong>{"Status: "}</strong> {&i.status}</p>
                  </div>
                </div>
            }
        } else {
            html! {
                <div class="columns">
                  <div class="column">
                    <p><strong>{"IP: "}</strong> {&i.interface}</p>
                  </div>
                  <div class="column">
                    <p><strong>{"Check type: "}</strong> {&i.check_method}</p>
                  </div>
                  <div class="column">
                    <p><strong>{"Status: "}</strong> {&i.status}</p>
                  </div>
                </div>
            }
        }
    }

    fn render_footer(&self) -> Html<Self> {
        if self.is_editing {
            html! {
                <>
                  <button class="button is-success"
                      onclick=|_| DeviceModalMsg::FinishEdit>
                    <span class="icon is-small">
                      <i class="far fa-save"></i>
                    </span>
                    <span>{"Finish edit"}</span>
                  </button>
                  <button class="button"
                      onclick=|e| DeviceModalMsg::HideModal>{"Cancel"}</button>
                </>
            }
        } else {
            html! {
                <>
                  <button class="button is-success"
                      onclick=|_| DeviceModalMsg::EditDevice>
                    <span class="icon is-small"><i class="far fa-edit"></i></span>
                    <span>{"Edit"}</span>
                  </button>
                  <button class="button"
                      onclick=|_| DeviceModalMsg::HideModal>{"Cancel"}</button>
                </>
            }
        }
    }
}

impl Component for DeviceModal {
    type Message = DeviceModalMsg;
    type Properties = DeviceModalProps;

    fn create(prop: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            device: prop.device,
            visible: prop.visible,
            on_close: prop.on_close,
            is_editing: false,
            fetch: FetchService::new(),
            task: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let mut console = ConsoleService::new();
        match msg {
            DeviceModalMsg::HideModal => {
                self.visible = false;
                self.on_close.emit(true);
            }
            DeviceModalMsg::EditDevice => {
                self.is_editing = true;
            }
            DeviceModalMsg::FinishEdit => self.is_editing = false,
            DeviceModalMsg::SubmitDevice(dev) => {
                let url = "https://r5fccfffwg.execute-api.eu-west-1.amazonaws.com/testing/devices";
                let device: Device = dev.into();

                let callback = self.link.send_back(
                    move |res: Response<Json<Result<(), Error>>>| {
                        let (meta, Json(_)) = res.into_parts();

                        if meta.status.is_success() {
                            DeviceModalMsg::SubmitSuccess
                        } else {
                            DeviceModalMsg::SubmitFail
                        }
                    },
                );

                let request = Request::post(url)
                    .body(Json(&device))
                    .expect("Panic while building a request.");

                self.task = Some(self.fetch.fetch(request, callback));
            }
            DeviceModalMsg::SubmitSuccess => {
                console.log("Succesfully sent");
            }
            DeviceModalMsg::SubmitFail => {
                console.log("Error sending");
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.visible = props.visible;
        self.device = props.device;
        true
    }
}

impl Renderable<DeviceModal> for DeviceModal {
    fn view(&self) -> Html<Self> {
        let modal_visible = if self.visible { "is-active" } else { "" };

        let overview = if self.is_editing {
            html! {
              <div class="field is-horizontal">
                <div class="field-label is-normal">
                  <label class="label">{"Name: "}</label>
                </div>
                <div class="control">
                  <input class="input" name="device-name" type="text" value=&self.device.name/>
                </div>

                <div class="field-label is-normal">
                  <label class="label">{"Location: "}</label>
                </div>
                <div class="control">
                  <input class="input" name="device-location" type="text" value=&self.device.location/>
                </div>
              </div>
            }
        } else {
            html! {
              <div class="columns">
                <div class="column">
                  <p><strong>{"Name: "}</strong> {&self.device.name}</p>
                </div>
                <div class="column">
                  <p><strong>{"Location: "}</strong> {&self.device.location}</p>
                </div>
              </div>
            }
        };

        let update_button = {
            if self.is_editing {
                html! {
                    <input type="submit" class="button is-primary"
                        value={"Update"}/>
                }
            } else {
                html! {
                    <>
                    </>
                }
            }
        };

        html! {
            <div class=("modal", modal_visible)>
                <div class="modal-background"></div>
                <div class="modal-card">
                <header class="modal-card-head">
                <p class="modal-card-title">{&self.device.name}</p>
                          <button class="delete"
                                  onclick=|_| DeviceModalMsg::HideModal></button>
                        </header>
                        <section class="modal-card-body">
                          <form onsubmit=|e| {
                              e.prevent_default();
                              let form_element: Element = e.target()
                                  .unwrap().try_into().unwrap();
                              DeviceModalMsg::SubmitDevice(FormData::from_element(&form_element).unwrap())
                          }>

                        <div class="tile is-ancestor">
                      <div class="is-vertical tile">
                        <div class="tile is-parent">
                          <nav class="level">
                            <div class="level-left"><div class="level-item"></div></div>

                            <div class="level-right">
                              <div class="level-item is-inline">
                        {update_button}
                                </div>
                            </div>
                          </nav>
                        </div>
                        <div class="tile is-parent">
                          <article class="tile is-child box is-12">
                            <p class="title">{"Overview"}</p>
                            {overview}
                          </article>
        </div>
                      </div>
                    </div>
                    <div class="tile is-ancestor">
                      <div class="tile is-parent is-4">
                        <article class="tile is-child box">
                          <p class="title">{"Events"}</p>
                          <p>{"No past  events"}</p>
                        </article>
                      </div>
                      <div class="tile is-parent is-8">
                        <article class="tile is-child box">
                          <p class="title">{"Interfaces"}</p>
                          {for self.device.interfaces.iter().map(|i| {
                          self.render_interfaces(i)
                          })}
                        </article>
                      </div>
                    </div>
                        </form>
                      </section>
                      <footer class="modal-card-foot">
                        {self.render_footer()}
                      </footer>
                    </div>
                  </div>
                }
    }
}
