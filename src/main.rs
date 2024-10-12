mod sensor_management;
mod prom;
mod protocols;
mod sensors;

#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::State;
use reqwest::Client;

use prometheus::{Encoder, TextEncoder};
use rocket::log::LogLevel;
use sensor_management::{SensorManager, init_sensor_manager, get_sensor_status, set_sensor_status};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/metrics")]
fn metrics() -> String {
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    format!("{}", String::from_utf8(buffer).unwrap())
}

#[post("/data/report", data = "<report>")]
async fn ecowitt_report(
    report: Form<protocols::ecowitt::Report<'_>>,
    promobj: &State<prom::PromObjects>,
    client: &State<Client>,
    manager: &State<SensorManager>,
) {
    let status = manager.get_status();
    let mut filtered_report = report.into_inner();

    if status.indoor_sensors {
        sensors::indoor::update_sensor_gauges(&filtered_report, promobj);
    } else {
        filtered_report.tempinf = None;
        filtered_report.humidityin = None;
        // Additional Indoor Sensor Metrics
        filtered_report.baromrelin = None;
        filtered_report.baromabsin = None;
        filtered_report.wh25batt = None;
        filtered_report.wh31batt = None;
    }

    if status.other_sensors {
        sensors::base::update_sensor_gauges(&filtered_report, promobj);
        sensors::outdoor_combined::update_sensor_gauges(&filtered_report, promobj);
    } else {
        // Outdoor Combined Sensor (WH80/WS80)
        filtered_report.tempf = None;
        filtered_report.humidity = None;
        filtered_report.winddir = None;
        filtered_report.windspeedmph = None;
        filtered_report.windgustmph = None;
        filtered_report.maxdailygust = None;
        filtered_report.solarradiation = None;
        filtered_report.uv = None;
        filtered_report.wh80batt = None;

        // Additional Temperature/Humidity Sensors (CH1-CH8)
        for i in 1..=8 {
            if let Some(field) = filtered_report.ch_temp_mut(i) {
                *field = None;
            }
            if let Some(field) = filtered_report.ch_humidity_mut(i) {
                *field = None;
            }
            if let Some(field) = filtered_report.ch_batt_mut(i) {
                *field = None;
            }
        }
    }

    if status.rain_gauge {
        sensors::raingauge::update_sensor_gauges(&filtered_report, promobj);
    } else {
        filtered_report.rainratein = None;
        filtered_report.eventrainin = None;
        filtered_report.hourlyrainin = None;
        filtered_report.dailyrainin = None;
        filtered_report.weeklyrainin = None;
        filtered_report.monthlyrainin = None;
        filtered_report.yearlyrainin = None;
        filtered_report.wh40batt = None;
    }

    if !status.soil_sensors {
        for i in 1..=8 {
            // let soil_moisture = format!("soilmoisture{}", i);
            // let soil_batt = format!("soilbatt{}", i);
            // Use if let Some() to safely update fields
            if let Some(field) = filtered_report.soil_moisture_mut(i) {
                *field = None;
            }
            if let Some(field) = filtered_report.soil_battery_mut(i) {
                *field = None;
            }
        }
    }

    if !status.is_all_disabled() {
        let forward_url = "http://cdnrtpdate.ecowitt.net/data/report/";
        match client
            .post(forward_url)
            .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .form(&filtered_report)
            .send()
            .await
        {
            Ok(response) => {
                if !response.status().is_success() {
                    println!("Failed to forward data. Status: {}", response.status());
                }
            }
            Err(e) => {
                println!("Failed to forward data: {}", e);
            }
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(rocket::Config::figment()
            .merge(("log_level", LogLevel::Normal)))
        .manage(prom::new())
        .manage(Client::new())
        .manage(init_sensor_manager())
        .mount("/", routes![
            index,
            metrics,
            ecowitt_report,
            get_sensor_status,
            set_sensor_status
        ])
}

// see: [False positive "main function not found" with rocket #5975](https://github.com/intellij-rust/intellij-rust/issues/5975)
// #[rocket::main]
// async fn main() {
//     let _ = rocket::build().mount("/", routes![index]).launch().await;
// }
