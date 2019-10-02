use crate::device::Device;
use crate::device_modal::DeviceModal;
use failure::Error;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Devices {
    devices: Vec<Device>,
    fetch: FetchService,
    link: ComponentLink<Devices>,
    task: Option<FetchTask>,
    modal_visible: bool,
    current_device: Option<Device>,
}

pub enum DevicesMsg {
    FetchOk(Vec<Device>),
    FetchFail,
    ShowDeviceModal(Device),
    HideDeviceModal,
    AddDeviceModal,
}

impl Component for Devices {
    type Message = DevicesMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let url =
            "https://r5fccfffwg.execute-api.eu-west-1.amazonaws.com/testing/devices";

        let mut dev = Devices {
            devices: Vec::new(),
            fetch: FetchService::new(),
            task: None,
            link,
            modal_visible: false,
            current_device: None,
        };

        let callback = dev.link.send_back(
            move |res: Response<Json<Result<Vec<Device>, Error>>>| {
                let (meta, Json(data)) = res.into_parts();
                if meta.status.is_success() {
                    match data {
                        Ok(d) => DevicesMsg::FetchOk(d),
                        Err(_) => DevicesMsg::FetchFail,
                    }
                } else {
                    DevicesMsg::FetchFail
                }
            },
        );
        let request = Request::get(url).body(Nothing).unwrap();
        dev.task = Some(dev.fetch.fetch(request, callback));

        dev
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            DevicesMsg::FetchFail => false,
            DevicesMsg::FetchOk(devices) => {
                self.devices = devices;
                self.task = None;
                true
            }
            DevicesMsg::ShowDeviceModal(device) => {
                self.modal_visible = true;
                self.current_device = Some(device);
                true
            }
            DevicesMsg::HideDeviceModal => {
                self.modal_visible = false;
                true
            }
            DevicesMsg::AddDeviceModal => true,
        }
    }
}

impl Renderable<Devices> for Devices {
    fn view(&self) -> Html<Self> {
        let devices_row = |d: &Device| {
            let interface_state = d.interface_summary();
            let dev = d.clone();
            html! {
                <tr onclick= |_| DevicesMsg::ShowDeviceModal(dev.clone())>
                    <td>{d.node_id}</td>
                    <td>{d.name.clone()}</td>
                    <td>{d.location.clone()}</td>
                    <td>{interface_state.0}{"/"}{interface_state.1}</td>
                    <td>
                      <button class="button is-dark is-small"
                        onclick=|_| DevicesMsg::AddDeviceModal>{"Add node"}
                      </button>
                    </td>
                </tr>
            }
        };

        let device_modal = match self.current_device.as_ref() {
            None => {
                html! {}
            }
            Some(dev) => {
                html! {
                    <DeviceModal: device=dev.clone() on_close=|_|DevicesMsg::HideDeviceModal visible=self.modal_visible/>
                }
            }
        };

        html! {
            <div>
              {device_modal}
              <div class="table-container">
                <h3>{"Devices"}</h3>
                <table class="table is-fullwidth is-bordered is-hoverable">
                  <thead class="thead-dark">
                    <tr>
                      <th>{"Device id"}</th>
                      <th>{"Device name"}</th>
                      <th>{"Device location"}</th>
                      <th>{"State"}</th>
                      <th>{"Actions"}</th>
                    </tr>
                  </thead>
                  <tbody>
                    {for self.devices.iter().map(devices_row)}
                  </tbody>
                </table>
              </div>
            </div>
        }
    }
}
