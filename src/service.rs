use std::{collections::HashSet, net::IpAddr, panic::Location, sync::Arc, time::Duration};

use crate::error::{LocationError, Result};
use dashmap::{mapref::one::Ref, DashMap};
use mdns_sd::{ServiceDaemon, ServiceEvent};
use tokio::time;
use tracing::{debug, debug_span};

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct MacAddr(pub String);

#[derive(Default, Debug, Clone)]
pub struct Services(Arc<DashMap<MacAddr, Service>>);

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
    pub async fn browse_services(&mut self) -> Result<()> {
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

                    self.0
                        .insert(MacAddr(std::str::from_utf8(mac_addr)?.to_string()), service);
                }
                ServiceEvent::ServiceRemoved(ty, full) => {
                    let mac = full.strip_suffix(&ty).unwrap();
                    self.0.remove(&MacAddr(mac.to_string()));
                }
                _ => (),
            }
        }

        Ok(())
    }

    pub async fn get(&self, mac: MacAddr) -> Result<Option<Ref<'_, MacAddr, Service>>> {
        let mut result = self.0.try_get(&mac);
        let mut interval = time::interval(Duration::from_millis(10));

        for _ in 0..10 {
            interval.tick().await;
            if result.is_present() {
                return Ok(Some(result.unwrap()));
            } else if result.is_absent() {
                return Ok(None);
            }

            result = self.0.try_get(&mac);
        }

        tracing::error!("failed to obtain lock on services map after 100ms");
        return Err(LocationError::LockFailed(mac).into());
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
