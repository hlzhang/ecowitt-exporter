use serde::Serialize;

#[derive(FromForm, Serialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct Report<'r> {
    // Basic station metrics, always returned
    // authentication key for ecowitt
    pub PASSKEY: &'r str,
    // general type of weather station
    pub stationtype: &'r str,
    // UTC timestamp
    pub dateutc: &'r str,
    // frequency used for wireless sensors
    pub freq: &'r str,
    // precise weather station model and firmware version
    pub model: &'r str,

    // Metrics provided by indoor sensor (WH25/WH32)
    // indoor temperature in fahrenheit
    pub tempinf: Option<f64>,
    // indoor humidity in percents
    pub humidityin: Option<i64>,
    // indoor relative pressure in in/Hg
    pub baromrelin: Option<f64>,
    // indoor absolute pressure in in/Hg
    pub baromabsin: Option<f64>,
    // WH25 sensor battery level
    pub wh25batt: Option<f64>,
    // WH31 sensor battery level
    pub wh31batt: Option<f64>,

    // Metrics provided by outdoor 1-in-6 combined sensor (WH80/WS80)
    // outdoor temperature in fahrenheit
    pub tempf: Option<f64>,
    // outdoor humidity in percents
    pub humidity: Option<i64>,
    // wind direction in degrees (north as 0)
    pub winddir: Option<i64>,
    // average wind speed in miles/hour
    pub windspeedmph: Option<f64>,
    // guest wind speed in miles/hour
    pub windgustmph: Option<f64>,
    // maximum daily gust speed in miles/hour
    pub maxdailygust: Option<f64>,
    // solar radiation in watt/m2
    pub solarradiation: Option<f64>,
    // UV index
    pub uv: Option<i64>,
    // WH80 sensor battery level
    pub wh80batt: Option<f64>,

    // Metrics provided by outdoor rain gauge (WH40)
    // Current rainfall in inch/h
    pub rainratein: Option<f64>,
    // Event rainfall in inch
    pub eventrainin: Option<f64>,
    // Hourly rainfall in inch
    pub hourlyrainin: Option<f64>,
    // Daily rainfall in inch
    pub dailyrainin: Option<f64>,
    // Weekly rainfall in inch
    pub weeklyrainin: Option<f64>,
    // Monthly rainfall in inch
    pub monthlyrainin: Option<f64>,
    // Yearly rainfall in inch
    pub yearlyrainin: Option<f64>,
    // WH40 sensor battery level
    pub wh40batt: Option<f64>,

    // Metrics provided by CH1-CH8 WH31 sensors
    // CH1 temperature in fahrenheit
    pub temp1f: Option<f64>,
    // CH1 humidity in percents
    pub humidity1: Option<i64>,
    // CH1 battery level
    pub batt1: Option<f64>,
    // CH2 temperature in fahrenheit
    pub temp2f: Option<f64>,
    // CH2 humidity in percents
    pub humidity2: Option<i64>,
    // CH2 battery level
    pub batt2: Option<f64>,
    // CH3 temperature in fahrenheit
    pub temp3f: Option<f64>,
    // CH3 humidity in percents
    pub humidity3: Option<i64>,
    // CH3 battery level
    pub batt3: Option<f64>,
    // CH4 temperature in fahrenheit
    pub temp4f: Option<f64>,
    // CH4 humidity in percents
    pub humidity4: Option<i64>,
    // CH4 battery level
    pub batt4: Option<f64>,
    // CH5 temperature in fahrenheit
    pub temp5f: Option<f64>,
    // CH5 humidity in percents
    pub humidity5: Option<i64>,
    // CH5 battery level
    pub batt5: Option<f64>,
    // CH6 temperature in fahrenheit
    pub temp6f: Option<f64>,
    // CH6 humidity in percents
    pub humidity6: Option<i64>,
    // CH6 battery level
    pub batt6: Option<f64>,
    // CH7 temperature in fahrenheit
    pub temp7f: Option<f64>,
    // CH7 humidity in percents
    pub humidity7: Option<i64>,
    // CH7 battery level
    pub batt7: Option<f64>,
    // CH8 temperature in fahrenheit
    pub temp8f: Option<f64>,
    // CH8 humidity in percents
    pub humidity8: Option<i64>,
    // CH8 battery level
    pub batt8: Option<f64>,

    // Metrics provided by CH1-CH8 WH51 Boden (soil moisture) sensors
    // CH1 soil moisture in percents
    pub soilmoisture1: Option<i64>,
    // CH1 battery level
    pub soilbatt1: Option<f64>,
    // CH2 soil moisture in percents
    pub soilmoisture2: Option<i64>,
    // CH2 battery level
    pub soilbatt2: Option<f64>,
    // CH3 soil moisture in percents
    pub soilmoisture3: Option<i64>,
    // CH3 battery level
    pub soilbatt3: Option<f64>,
    // CH4 soil moisture in percents
    pub soilmoisture4: Option<i64>,
    // CH4 battery level
    pub soilbatt4: Option<f64>,
    // CH5 soil moisture in percents
    pub soilmoisture5: Option<i64>,
    // CH5 battery level
    pub soilbatt5: Option<f64>,
    // CH6 soil moisture in percents
    pub soilmoisture6: Option<i64>,
    // CH6 battery level
    pub soilbatt6: Option<f64>,
    // CH7 soil moisture in percents
    pub soilmoisture7: Option<i64>,
    // CH7 battery level
    pub soilbatt7: Option<f64>,
    // CH8 soil moisture in percents
    pub soilmoisture8: Option<i64>,
    // CH8 battery level
    pub soilbatt8: Option<f64>,
}

impl<'r> Report<'r> {
    pub fn ch_temp_mut(&mut self, i: usize) -> Option<&mut Option<f64>> {
        match i {
            1 => Some(&mut self.temp1f),
            2 => Some(&mut self.temp2f),
            3 => Some(&mut self.temp3f),
            4 => Some(&mut self.temp4f),
            5 => Some(&mut self.temp5f),
            6 => Some(&mut self.temp6f),
            7 => Some(&mut self.temp7f),
            8 => Some(&mut self.temp8f),
            _ => None,
        }
    }

    pub fn ch_humidity_mut(&mut self, i: usize) -> Option<&mut Option<i64>> {
        match i {
            1 => Some(&mut self.humidity1),
            2 => Some(&mut self.humidity2),
            3 => Some(&mut self.humidity3),
            4 => Some(&mut self.humidity4),
            5 => Some(&mut self.humidity5),
            6 => Some(&mut self.humidity6),
            7 => Some(&mut self.humidity7),
            8 => Some(&mut self.humidity8),
            _ => None,
        }
    }

    pub fn ch_batt_mut(&mut self, i: usize) -> Option<&mut Option<f64>> {
        match i {
            1 => Some(&mut self.batt1),
            2 => Some(&mut self.batt2),
            3 => Some(&mut self.batt3),
            4 => Some(&mut self.batt4),
            5 => Some(&mut self.batt5),
            6 => Some(&mut self.batt6),
            7 => Some(&mut self.batt7),
            8 => Some(&mut self.batt8),
            _ => None,
        }
    }

    // Add methods to mutably access soil moisture and battery fields
    pub fn soil_moisture_mut(&mut self, i: usize) -> Option<&mut Option<i64>> {
        match i {
            1 => Some(&mut self.soilmoisture1),
            2 => Some(&mut self.soilmoisture2),
            3 => Some(&mut self.soilmoisture3),
            4 => Some(&mut self.soilmoisture4),
            5 => Some(&mut self.soilmoisture5),
            6 => Some(&mut self.soilmoisture6),
            7 => Some(&mut self.soilmoisture7),
            8 => Some(&mut self.soilmoisture8),
            _ => None,
        }
    }

    pub fn soil_battery_mut(&mut self, i: usize) -> Option<&mut Option<f64>> {
        match i {
            1 => Some(&mut self.soilbatt1),
            2 => Some(&mut self.soilbatt2),
            3 => Some(&mut self.soilbatt3),
            4 => Some(&mut self.soilbatt4),
            5 => Some(&mut self.soilbatt5),
            6 => Some(&mut self.soilbatt6),
            7 => Some(&mut self.soilbatt7),
            8 => Some(&mut self.soilbatt8),
            _ => None,
        }
    }
}
