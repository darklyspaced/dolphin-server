use std::{
    collections::{HashMap, HashSet},
    net::IpAddr,
    sync::Arc,
};

use anyhow::Result;
use dashmap::DashMap;
use mdns_sd::{ServiceDaemon, ServiceEvent};
use tracing::{debug, debug_span};

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct MacAddr(pub String);

#[derive(Default, Debug)]
pub struct Services(HashMap<MacAddr, Service>);

pub enum Event {
    New { key: MacAddr, service: Service },
    Remove(MacAddr),
}

impl Services {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Infinitely running daemon that continuously adds
    pub async fn browse_services(services: Arc<DashMap<MacAddr, Service>>) -> Result<()> {
        let mdns = ServiceDaemon::new()?;
        let service_type = "_dolphin._tcp.local.";
        let receiver = mdns.browse(service_type)?;

        let browse = debug_span!("browse");
        let _ = browse.enter();

        while let Ok(event) = receiver.recv_async().await {
            match event {
                ServiceEvent::ServiceResolved(info) => {
                    let Some(property) = info.get_property("mac") else {
                        panic!("invalid service definition");
                    };
                    let Some(mac_addr) = property.val() else {
                        panic!("invalid service definition");
                    };

                    let (addrs, port) = (info.get_addresses(), info.get_port());
                    let service = Service::from(addrs, port);

                    debug!("connect at {addrs:?}/{port}");

                    services.insert(MacAddr(std::str::from_utf8(mac_addr)?.to_string()), service);
                }
                ServiceEvent::ServiceRemoved(ty, full) => {
                    let mac = full.strip_suffix(&ty).unwrap();
                    services.remove(&MacAddr(mac.to_string()));
                }
                _ => (),
            }
        }

        Ok(())
    }

    fn add(&mut self, mac_addr: MacAddr, service: Service) {
        self.0.insert(mac_addr, service);
    }

    pub async fn get_services(&self) -> Vec<&Service> {
        self.0.values().collect::<Vec<_>>()
    }
}

#[derive(Debug)]
pub struct Service {
    pub addr: IpAddr,
    pub port: u16,
}

impl Service {
    pub fn from(addrs: &HashSet<IpAddr>, port: u16) -> Self {
        let addr = addrs.iter().next().expect("fewer than one addr");
        Self { addr: *addr, port }
    }
}
