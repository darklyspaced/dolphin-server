use std::{
    collections::{HashMap, HashSet},
    net::IpAddr,
};

use mdns_sd::{ServiceDaemon, ServiceEvent};
use tracing::{debug, debug_span};

#[derive(Debug, Eq, Hash, PartialEq)]
struct MacAddr(String);

#[derive(Default, Debug)]
struct Services(HashMap<MacAddr, Service>);

impl Services {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn add(&mut self, mac_addr: MacAddr, service: Service) {
        self.0.insert(mac_addr, service);
    }
}

#[derive(Debug)]
struct Service {
    addr: IpAddr,
    port: u16,
}

impl Service {
    pub fn from(addrs: &HashSet<IpAddr>, port: u16) -> Self {
        let addr = addrs.iter().next().expect("fewer than one addr");
        Self { addr: *addr, port }
    }
}

async fn browse_dolphin() {
    let mdns = ServiceDaemon::new().expect("failed to create daemon");
    let service_type = "_dolphin._tcp.local.";
    let receiver = mdns.browse(service_type).expect("failed to browse");

    let browse = debug_span!("browse");
    let _ = browse.enter();

    while let Ok(event) = receiver.recv_async().await {
        match event {
            ServiceEvent::ServiceResolved(info) => {
                let (addrs, port) = (info.get_addresses(), info.get_port());
                debug!("connect at {addrs:?}/{port}");

                let service = Service::from(addrs, port);
            }
            _other_event => {}
        }
    }
}
