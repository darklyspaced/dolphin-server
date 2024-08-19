use std::{collections::HashSet, net::IpAddr, sync::Arc, time::Duration};

use crate::{
    error::{LocationError, Result},
    locations::Location,
};
use dashmap::{mapref::one::Ref, DashMap};
use mdns_sd::{ServiceDaemon, ServiceEvent};
use tokio::{io::AsyncReadExt, net::TcpStream, time};
use tracing::debug_span;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MacAddr(pub String);

#[derive(Default, Debug, Clone)]
pub struct Services(pub Arc<DashMap<MacAddr, Service>>);

impl Services {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Infinitely running daemon that continuously adds services to it's internal
    /// list of services
    pub async fn browse_services(&mut self) -> Result<()> {
        let mdns = ServiceDaemon::new()?;
        let service_type = "_dolphin._tcp.local.";
        let receiver = mdns.browse(service_type)?;

        let browse = debug_span!("browse");
        let _ = browse.enter();

        while let Ok(event) = receiver.recv_async().await {
            match event {
                ServiceEvent::ServiceResolved(info) => {
                    // gets the mac address (stored in list of properties for service)
                    let Some(property) = info.get_property("mac") else {
                        panic!("invalid service definition");
                    };
                    let Some(mac_addr) = property.val() else {
                        panic!("invalid service definition");
                    };

                    let (addrs, port) = (info.get_addresses(), info.get_port());
                    let service = Service::from(addrs, port);
                    let mac_addr = MacAddr(std::str::from_utf8(mac_addr)?.to_string());

                    tracing::info!("new service at {addrs:?}/{port} with {mac_addr:?}");
                    self.add_service(mac_addr, service).await;
                }
                ServiceEvent::ServiceRemoved(ty, full) => {
                    // extract mac address from title of service
                    let mac = full.strip_suffix(&ty).unwrap();

                    tracing::info!("deregistered new laptop with address {mac}");

                    self.0.remove(&MacAddr(mac.to_string()));
                }
                _ => (),
            }
        }

        Ok(())
    }

    pub async fn add_service(&mut self, mac: MacAddr, service: Service) {
        self.0.insert(dbg!(mac.clone()), service);
        dbg!(self.get(mac).await.unwrap());
    }

    /// Returns the addr and port of the mac with the specific mac addr
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
        Err(LocationError::LockFailed(mac.clone()).into())
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

    /// Fallible function that attempts to connect with the service and then get the location of
    /// the client
    pub async fn try_get_loc(&self) -> Result<Location> {
        let mut stream =
            TcpStream::connect(format!("{}:{}", self.addr, &self.port.to_string())).await?;

        let mut loc = String::new();
        stream.read_to_string(&mut loc).await?;

        Ok(Location(loc))
    }
}
