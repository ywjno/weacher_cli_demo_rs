use chrono::{FixedOffset, TimeZone};
use exitfailure::ExitFailure;
use reqwest::{StatusCode, Url};
use serde_derive::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    lon: f32,
    lat: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Main {
    temp: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32,
    pressure: u32,
    humidity: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed: f32,
    deg: u32,
    #[serde(default)]
    gust: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Clouds {
    all: u32,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
struct Rain {
    #[serde(rename = "1h")]
    one_hour: f32,
    #[serde(rename = "3h")]
    three_hours: f32,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
struct Snow {
    #[serde(rename = "1h")]
    one_hour: f32,
    #[serde(rename = "3h")]
    three_hours: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Sys {
    r#type: u32,
    id: u32,
    #[serde(default)]
    message: String,
    country: String,
    sunrise: u32,
    sunset: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Openweather {
    coord: Coord,
    weather: Vec<Weather>,
    base: String,
    main: Main,
    visibility: u32,
    wind: Wind,
    clouds: Clouds,
    rain: Option<Rain>,
    snow: Option<Snow>,
    dt: i64,
    sys: Sys,
    timezone: i32,
    id: u32,
    name: String,
    cod: u32,
}

impl Openweather {
    async fn get(city_name: &String) -> Result<Self, ExitFailure> {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={}&appid=4845f22236e074cdac59ae174aa580a3",
            city_name
        );
        let url = Url::parse_with_params(&*url, &[("units", "metric"), ("lang", "ja")])?;
        let resp = reqwest::get(url).await?;
        if resp.status() == StatusCode::OK {
            Ok(resp.json::<Openweather>().await?)
        } else {
            Err(failure::err_msg(resp.text().await?).into())
        }
    }
}

#[derive(StructOpt, Debug)]
struct Input {
    city: String,
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let input = Input::from_args();
    let resp = Openweather::get(&input.city).await?;
    if resp.name == input.city {
        println!("城市: {}", resp.name);
    } else {
        println!("城市: {}({})", resp.name, input.city);
    }
    println!(
        "当前时刻: {}",
        FixedOffset::east(resp.timezone).timestamp(resp.dt, 0)
    );
    println!("当前温度: {}°C", resp.main.temp);
    println!("最高温度: {}°C", resp.main.temp_max);
    println!("最低温度： {}°C", resp.main.temp_min);
    println!("湿度: {}%", resp.main.humidity);
    Ok(())
}
