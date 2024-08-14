use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use sqlx::MySqlPool;
use tokio::sync::Mutex;

use crate::service::MacAddr;

#[derive(PartialEq)]
pub struct Location(pub String);

// PERF: calls clone on every property
#[derive(Clone)]
pub struct Locations(pub Arc<Mutex<InnerLocs>>);

impl Locations {
    pub async fn new(pool: MySqlPool) -> Self {
        Self(Arc::new(Mutex::new(InnerLocs::new(pool).await)))
    }

    pub async fn update_location(&self, mac: &MacAddr, curr_loc: Location) {
        self.0.lock().await.update_location(mac, curr_loc);
    }

    pub async fn push_updates(self, pool: MySqlPool) {
        self.0.lock().await.push_updates(pool).await;
    }
}

#[derive(Default)]
pub struct InnerLocs {
    pub locations: HashMap<MacAddr, Option<Location>>,
    changed: HashSet<MacAddr>,
}

impl InnerLocs {
    /// Populates with all known locations by default
    pub async fn new(pool: MySqlPool) -> Self {
        let mut locations = HashMap::new();
        let locs = sqlx::query!(
            "
SELECT * FROM locations;
            "
        )
        .fetch_all(&pool)
        .await
        .unwrap();

        for loc in locs {
            locations.insert(
                MacAddr(loc.mac),
                if !loc.bssid.is_empty() {
                    Some(Location(loc.bssid))
                } else {
                    None
                },
            );
        }

        Self {
            locations,
            ..Default::default()
        }
    }

    /// Updates the location of a laptop if changed and marks it to be pushed to database
    pub fn update_location(&mut self, mac: &MacAddr, curr_loc: Location) {
        if let Some(Some(loc)) = self.locations.get(mac) {
            if *loc == curr_loc {
                // don't change location if it exists in set and is same as current
                return;
            }
        }

        self.locations.insert(mac.clone(), Some(curr_loc));
        self.changed.insert(mac.clone());
    }

    /// Pushes all changed locations to the database
    pub async fn push_updates(&mut self, pool: MySqlPool) {
        for changed in self.changed.drain() {
            let Some(Some(new_loc)) = self.locations.get(&changed) else {
                unreachable!("location supposedly was changed yet doesn't exist")
            };

            sqlx::query!(
                "
INSERT INTO locations (mac, bssid)
VALUES (?, ?)
ON DUPLICATE KEY UPDATE
bssid = VALUES(bssid);
                ",
                changed.0,
                new_loc.0
            )
            .execute(&pool)
            .await
            .expect("failed to update location");
        }
    }
}
