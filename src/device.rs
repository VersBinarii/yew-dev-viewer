use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;
use stdweb::web::{FormData, FormDataEntry};
use uuid::Uuid;
use yew::prelude::*;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum CheckMethod {
    Http,
    Ping,
    SipPing,
}

impl std::fmt::Display for CheckMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckMethod::Http => write!(f, "{}", "Http"),
            CheckMethod::SipPing => write!(f, "{}", "SipPing"),
            CheckMethod::Ping => write!(f, "{}", "Ping"),
        }
    }
}

impl FromStr for CheckMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "PING" | "ping" | "Ping" => Ok(CheckMethod::Ping),
            "HTTP" | "http" | "Http" => Ok(CheckMethod::Http),
            "SipPing" | "sip-ping" | "sipping" | "SIP-PING" => {
                Ok(CheckMethod::SipPing)
            }
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Copy, Clone)]
pub enum InterfaceStatus {
    Up,
    Down,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Interface {
    pub check_method: CheckMethod,
    pub interface: String,
    pub status: InterfaceStatus,
}

impl std::fmt::Display for InterfaceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterfaceStatus::Down => write!(f, "{}", "Down"),
            InterfaceStatus::Up => write!(f, "{}", "Up"),
        }
    }
}

impl FromStr for InterfaceStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "Up" | "up" | "UP" => Ok(InterfaceStatus::Up),
            "Down" | "DOWN" | "down" => Ok(InterfaceStatus::Down),
            _ => Err(()),
        }
    }
}

impl std::default::Default for Interface {
    fn default() -> Self {
        Self {
            check_method: CheckMethod::Ping,
            interface: String::new(),
            status: InterfaceStatus::Down,
        }
    }
}

#[derive(Properties, Serialize, Deserialize, Debug, Clone)]
pub struct Device {
    pub node_id: Uuid,
    pub name: String,
    pub location: String,
    pub interfaces: Vec<Interface>,
}

impl Device {
    pub fn interface_summary(&self) -> (usize, usize) {
        let up = self
            .interfaces
            .iter()
            .filter(|i| i.status == InterfaceStatus::Up)
            .count();
        let total = self.interfaces.len();

        (up, total)
    }
}

impl std::default::Default for Device {
    fn default() -> Self {
        Self {
            name: String::from(""),
            node_id: Uuid::new_v4(),
            location: String::from(""),
            interfaces: Vec::new(),
        }
    }
}

impl From<FormData> for Device {
    fn from(fd: FormData) -> Self {
        let name = match fd.get("device-name").unwrap() {
            FormDataEntry::String(dev_name) => dev_name,
            _ => unreachable!(),
        };
        let location = match fd.get("device-location").unwrap() {
            FormDataEntry::String(dev_location) => dev_location,
            _ => unreachable!(),
        };

        let iface_address = fd.get_all("iface-address");
        let iface_check_method = fd.get_all("iface-check-method");

        let interfaces: Vec<Interface> = iface_address
            .iter()
            .zip(iface_check_method.iter())
            .map(|i| match i {
                (
                    FormDataEntry::String(address),
                    FormDataEntry::String(check_method),
                ) => (address.clone(), check_method.parse().unwrap()),
                (_, _) => (String::new(), CheckMethod::Ping),
            })
            .map(|(a, cm)| Interface {
                interface: a,
                check_method: cm,
                ..Default::default()
            })
            .collect();

        Self {
            name,
            location,
            interfaces,
            ..Device::default()
        }
    }
}
