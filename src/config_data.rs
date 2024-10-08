use std::collections::HashMap;

use sqlx::MySqlPool;

#[derive(Debug, Clone, Default)]
pub struct Ap {
    headers: [String; 2],
    pub data: HashMap<String, String>,
}
#[derive(Debug, Clone, Default)]
pub struct Trolleys {
    headers: [String; 3],
    /// <Mac, (Device Name, Trolley)>
    pub data: HashMap<String, (String, String)>,
}

pub trait Config: IntoIterator {
    /// Fetches the data from the database
    async fn get_latest_data(&mut self, pool: MySqlPool);
    async fn push_updates(&mut self, pool: MySqlPool);
}

impl Config for Ap {
    async fn get_latest_data(&mut self, pool: MySqlPool) {
        let mappings = sqlx::query!(
            "
SELECT * FROM access_points;
            "
        )
        .fetch_all(&pool)
        .await
        .unwrap();

        self.data.clear();
        for x in mappings {
            self.data.insert(x.bssid, x.room_name);
        }
    }

    async fn push_updates(&mut self, pool: MySqlPool) {
        for (key, ap) in self.data.iter() {
            let mappings = sqlx::query!(
                "
UPDATE access_points
SET bssid = ?, room_name = ?;
                ",
                key,
                ap
            )
            .execute(&pool)
            .await;
        }
    }
}

impl Config for Trolleys {
    async fn get_latest_data(&mut self, pool: MySqlPool) {
        let mappings = sqlx::query!(
            "
SELECT * FROM laptops;
            "
        )
        .fetch_all(&pool)
        .await
        .unwrap();

        self.data.clear();
        for x in mappings {
            self.data.insert(
                x.mac,
                (
                    x.device_name.unwrap_or(String::new()),
                    x.trolley.unwrap_or(String::new()),
                ),
            );
        }
    }

    // TODO: make it update everytime the hashmap is changed for individual values instead of all
    // at once
    async fn push_updates(&mut self, pool: MySqlPool) {
        sqlx::query!("DELETE FROM laptops")
            .execute(&pool)
            .await
            .unwrap();
        for (key, (device_name, trolley)) in self.data.iter() {
            sqlx::query!(
                "
INSERT INTO laptops (mac, device_name, trolley)
VALUES (?, ?, ?);
                ",
                key,
                device_name,
                trolley
            )
            .execute(&pool)
            .await
            .unwrap();
        }
    }
}

impl Trolleys {
    pub fn new() -> Self {
        Self {
            headers: [
                String::from("mac"),
                String::from("device_name"),
                String::from("trolley"),
            ],
            ..Default::default()
        }
    }
}

impl Ap {
    pub fn new() -> Self {
        Self {
            headers: [String::from("bssid"), String::from("room_name")],
            ..Default::default()
        }
    }
}

impl IntoIterator for Trolleys {
    type Item = [String; 3];
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.headers.clone()]
            .into_iter()
            .chain(
                self.data
                    .clone()
                    .into_iter()
                    .map(|(mac, (name, trolley))| [mac, name, trolley]),
            )
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl IntoIterator for Ap {
    type Item = [String; 2];
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.headers]
            .into_iter()
            .chain(self.data.into_iter().map(|(bssid, name)| [bssid, name]))
            .collect::<Vec<_>>()
            .into_iter()
    }
}
