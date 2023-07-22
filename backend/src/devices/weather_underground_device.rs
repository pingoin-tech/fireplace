use std::collections::BTreeMap;

use super::get_device_from_list;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use chrono::Utc;
use fireplace::{devices::{Device, DeviceType, subdevices::SubDevice}, eventhandler::Value};
use serde::Deserialize;

#[get("/weatherstation/updateweatherstation.php")]
pub async fn weatherstation(req: HttpRequest, data: web::Query<WuData>) -> impl Responder {
    let mut vals: BTreeMap<String, Value> = BTreeMap::new();
    vals.insert(
        "firmware".to_string(),
        Value::String(data.softwaretype.clone()),
    );

    if let Some(humidity) = data.humidity {
        vals.insert("outdoor-humidity".to_string(), Value::Number(humidity));
    }

    if let Some(humidity) = data.indoor_humidity {
        vals.insert("indoor-humidity".to_string(), Value::Number(humidity));
    }

    if let Some(winddir) = data.winddir {
        vals.insert("wind-dir".to_string(), Value::Number(winddir));
    }

    if let Some(uv_index) = data.uv_index {
        vals.insert("uv-index".to_string(), Value::Number(uv_index as f32));
    }

    if let Some(solar) = data.solar_radiation {
        vals.insert("solar-radiation".to_string(), Value::Number(solar));
    }

    if let Some(indoor_temp_f) = data.indoor_temp_f {
        vals.insert(
            "indoor-temp".to_string(),
            Value::Number(fahrenheit_in_celsius(indoor_temp_f)),
        );
    }

    if let Some(temp_f) = data.temp_f {
        vals.insert(
            "outdoor-temp".to_string(),
            Value::Number(fahrenheit_in_celsius(temp_f)),
        );
    }

    if let Some(rain) = data.rain_in {
        vals.insert(
            "rain".to_string(),
            Value::Number(inch_in_mm(rain)),
        );
    }

    if let Some(rain) = data.daily_rain_in {
        vals.insert(
            "daily-rain".to_string(),
            Value::Number(inch_in_mm(rain)),
        );
    }

    if let Some(rain) = data.monthly_rain_in {
        vals.insert(
            "monthly-rain".to_string(),
            Value::Number(inch_in_mm(rain)),
        );
    }

    if let Some(rain) = data.weekly_rain_in {
        vals.insert(
            "weekly-rain".to_string(),
            Value::Number(inch_in_mm(rain)),
        );
    }

    if let Some(dew_f) = data.dew_point_f {
        vals.insert(
            "dew-point".to_string(),
            Value::Number(fahrenheit_in_celsius(dew_f)),
        );
    }


    if let Some(temp_f) = data.wind_chill_f {
        vals.insert(
            "wind-chill".to_string(),
            Value::Number(fahrenheit_in_celsius(temp_f)),
        );
    }

    if let Some(wind) = data.wind_gust_mph {
        vals.insert(
            "wind-gust".to_string(),
            Value::Number(mph_in_kmh(wind)),
        );
    }

    if let Some(wind) = data.wind_speed_mph {
        vals.insert(
            "wind-speed".to_string(),
            Value::Number(mph_in_kmh(wind)),
        );
    }

    if let Some(baro) = data.barom_in {
        vals.insert(
            "baro-hpa".to_string(),
            Value::Number(inch_hg_in_hpa(baro)),
        );
    }

    if let Some(baro) = data.abs_barom_in {
        vals.insert(
            "abs-baro-hpa".to_string(),
            Value::Number(inch_hg_in_hpa(baro)),
        );
    }

let mut subs: BTreeMap<String, SubDevice> = BTreeMap::new();
for (key, v) in vals{
    subs.insert(key, SubDevice::Sensor(v));
}
    get_device_from_list(
        data.id.clone(),
        |dev| {
            dev.last_message = Utc::now();
            dev.last_data = Utc::now();
            dev.subdevices = subs.clone();
        },
        |list| {
            let mut dev = Device::default();
            dev.device_type=DeviceType::WeatherUndergrundDevice;
            dev.id = data.id.clone();
            if let Some(val) = req.peer_addr() {
                dev.ip = val.ip().to_string();
            };
            dev.last_message = Utc::now();
            dev.last_data = Utc::now();
            dev.subdevices = subs.clone();

            list.push(dev);
        },
        (),
    );
    HttpResponse::Ok().body("error")
}

fn fahrenheit_in_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn inch_in_mm(inch:f32)->f32{
    inch*25.4
}

fn mph_in_kmh(mph:f32)->f32{
    mph*1.60934400
}

fn inch_hg_in_hpa(inch:f32)->f32{
    inch *33.863787
}

#[derive(Deserialize, Debug)]
pub struct WuData {
    #[serde(rename = "ID")]
    id: String,
    //PASSWORD:String,
    #[serde(rename = "indoortempf")]
    indoor_temp_f: Option<f32>,
    #[serde(rename = "tempf")]
    temp_f: Option<f32>,
    #[serde(rename = "dewptf")]
    dew_point_f: Option<f32>,
    #[serde(rename = "windchillf")]
    wind_chill_f: Option<f32>,
    #[serde(rename = "indoorhumidity")]
    indoor_humidity: Option<f32>,
    humidity: Option<f32>,
    #[serde(rename = "windspeedmph")]
    wind_speed_mph: Option<f32>,
    #[serde(rename = "windgustmph")]
    wind_gust_mph: Option<f32>,
    winddir: Option<f32>,
    #[serde(rename = "absbaromin")]
    abs_barom_in: Option<f32>,
    #[serde(rename = "baromin")]
    barom_in: Option<f32>,
    #[serde(rename = "rainin")]
    rain_in: Option<f32>,
    #[serde(rename = "dailyrainin")]
    daily_rain_in: Option<f32>,
    #[serde(rename = "weeklyrainin")]
    weekly_rain_in: Option<f32>,
    #[serde(rename = "monthlyrainin")]
    monthly_rain_in: Option<f32>,
    #[serde(rename = "solarradiation")]
    solar_radiation: Option<f32>,
    #[serde(rename = "UV")]
    uv_index: Option<u8>,
    softwaretype: String,
}

