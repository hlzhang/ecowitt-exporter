// src/sensor_management.rs

use parking_lot::RwLock;
use rocket::State;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorStatus {
    pub indoor_sensors: bool,
    pub rain_gauge: bool,
    pub soil_sensors: bool,
    pub other_sensors: bool,
}

impl Default for SensorStatus {
    fn default() -> Self {
        SensorStatus {
            indoor_sensors: true,
            rain_gauge: true,
            soil_sensors: true,
            other_sensors: true,
        }
    }
}

pub struct SensorManager {
    status: Arc<RwLock<SensorStatus>>,
}

impl SensorManager {
    pub fn new() -> Self {
        let status = match fs::read_to_string("sensor_status.json") {
            Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
            Err(_) => SensorStatus::default(),
        };
        SensorManager {
            status: Arc::new(RwLock::new(status)),
        }
    }

    pub fn get_status(&self) -> SensorStatus {
        self.status.read().clone()
    }

    pub fn set_status(&self, new_status: SensorStatus) {
        let mut status = self.status.write();
        *status = new_status;
        // Release the lock before saving to file
        drop(status);
        self.save_status();
    }

    fn save_status(&self) {
        let status = self.status.read();
        match serde_json::to_string(&*status) {
            Ok(json) => {
                if let Err(e) = fs::write("sensor_status.json", json) {
                    eprintln!("Failed to save sensor status: {}", e);
                }
            },
            Err(e) => eprintln!("Failed to serialize sensor status: {}", e),
        }
    }
}

#[get("/sensor_status")]
pub fn get_sensor_status(manager: &State<SensorManager>) -> Json<SensorStatus> {
    Json(manager.get_status())
}

#[post("/sensor_status", data = "<new_status>")]
pub fn set_sensor_status(new_status: Json<SensorStatus>, manager: &State<SensorManager>) -> Json<SensorStatus> {
    manager.set_status(new_status.into_inner());
    Json(manager.get_status())
}

// This function can be used in your main.rs to initialize the SensorManager
pub fn init_sensor_manager() -> SensorManager {
    SensorManager::new()
}
