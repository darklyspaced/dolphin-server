use std::time::Duration;

use crate::{locations::Locations, service::Services};

use sqlx::MySqlPool;
use tokio::time::interval;

/// how often the laptops should be asked for their location (s^-1). technically the max length is ~2
/// * INTERVAL due the randomness of how entries are returned from HashMaps.
const INTERVAL: u64 = 1;
/// how many laptops per batch of requests. server can probably handle around 100
const BATCH_SIZE: usize = 5;
/// how long to wait between each batch of requests (ms^-1)
const PADDING: u64 = 10;

pub struct LoadBalancer {
    services: Services,
    locations: Locations,
    pool: MySqlPool,
}

impl LoadBalancer {
    pub fn new(services: Services, pool: MySqlPool, locations: Locations) -> Self {
        Self {
            services,
            locations,
            pool,
        }
    }

    pub fn run(self) {
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(INTERVAL));
            loop {
                interval.tick().await;
                // SAFETY: `.iter()` only takes a reference so taking a mutable reference later
                // does not produce a deadlock

                // makes sure that the proxy/server is not bombarded with requests
                let laptops = self.services.0.iter().count();
                let batches = laptops / BATCH_SIZE;

                let mut iter = self.services.0.iter();
                let mut padding = tokio::time::interval(Duration::from_millis(PADDING));

                for _ in 0..batches {
                    padding.tick().await;

                    for svc in iter.by_ref().take(BATCH_SIZE) {
                        match svc.try_get_loc().await {
                            Ok(loc) => {
                                self.locations.update_location(svc.key(), loc).await;
                            }
                            Err(_) => {
                                tracing::warn!(
                                    "removed {:?}. its service could not be accessed",
                                    &svc.key()
                                );
                                let _ = self.services.0.remove(svc.key());
                            }
                        };
                    }
                }

                tokio::spawn(self.locations.clone().push_updates(self.pool.clone()));
            }
        });
    }
}
