use rocket::State;

use crate::prom::PromObjects;
use crate::protocols::ecowitt::Report;
use crate::sensors::utils::{press_inhg_to_pa, temp_f_to_c};

pub fn update_sensor_gauges(report: &Report<'_>, prom: &State<PromObjects>) {
    // Temperature
    match report.tempinf {
        Some(temp) => {
            prom.temperature_f.with_label_values(&["indoor"]).set(temp);
            prom.temperature_c
                .with_label_values(&["indoor"])
                .set(temp_f_to_c(temp));
        }
        _ => {
            prom.temperature_f
                .remove_label_values(&["indoor"])
                .unwrap_or(());
            prom.temperature_c
                .remove_label_values(&["indoor"])
                .unwrap_or(());
        }
    };

    // Pressure
    match report.baromabsin {
        Some(press) => {
            prom.pressure_inhg
                .with_label_values(&["indoor", "abs"])
                .set(press);
            prom.pressure_pa
                .with_label_values(&["indoor", "abs"])
                .set(press_inhg_to_pa(press));
        }
        _ => {
            prom.pressure_inhg
                .remove_label_values(&["indoor", "abs"])
                .unwrap_or(());
            prom.pressure_pa
                .remove_label_values(&["indoor", "abs"])
                .unwrap_or(());
        }
    };
    match report.baromrelin {
        Some(press) => {
            prom.pressure_inhg
                .with_label_values(&["indoor", "rel"])
                .set(press);
            prom.pressure_pa
                .with_label_values(&["indoor", "rel"])
                .set(press_inhg_to_pa(press));
        }
        _ => {
            prom.pressure_inhg
                .remove_label_values(&["indoor", "rel"])
                .unwrap_or(());
            prom.pressure_pa
                .remove_label_values(&["indoor", "rel"])
                .unwrap_or(());
        }
    };

    // Humidity
    match report.humidityin {
        Some(hum) => {
            prom.humidity.with_label_values(&["indoor"]).set(hum);
        }
        _ => {
            prom.humidity.remove_label_values(&["indoor"]).unwrap_or(());
        }
    };

    // Battery
    match report.wh25batt {
        Some(batt) => {
            prom.battery.with_label_values(&["wh25"]).set(batt);
        }
        _ => {
            prom.battery.remove_label_values(&["wh25"]).unwrap_or(());
        }
    };
}
