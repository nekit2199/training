use serde::Serialize;
use std::fs::File;
use std::io;
use std::io::Write;

#[derive(Serialize)]
struct DataWeather {
    temperature_fact: f32,
    humidity: f32,
    wind: f32,
    temperature_like: f32,
}
impl DataWeather {
    fn new_json(self) {
        let json = serde_json::to_string_pretty(&self).unwrap();
        let mut file = File::create("weather.json").expect("Не удалось создать файл");

        // 3. Записываем строку в файл, предварительно переведя её в байты (.as_bytes()) [1]
        file.write_all(json.as_bytes())
            .expect("Не удалось записать данные в файл");
    }
}

fn main() {
    let mut temperature_fact_temp = String::new();
    let mut humidity_temp = String::new();
    let mut wind_temp = String::new();
    println!("Введите температуру на градуснике");
    io::stdin()
        .read_line(&mut temperature_fact_temp)
        .expect("Введите число");
    println!("Введите процент влажности");
    io::stdin()
        .read_line(&mut humidity_temp)
        .expect("Введите число");
    println!("Введите скорость ветра");
    io::stdin()
        .read_line(&mut wind_temp)
        .expect("Введите число");
    let temperature_fact: f32 = temperature_fact_temp
        .trim()
        .replace(",", ".")
        .parse::<f32>()
        .expect("");
    let humidity: f32 = humidity_temp
        .trim()
        .replace(",", ".")
        .parse::<f32>()
        .expect("");
    let wind: f32 = wind_temp.trim().replace(",", ".").parse::<f32>().expect("");
    let temperature_like = temp_feels_like(temperature_fact, humidity, wind);

    let data_weather = DataWeather {
        temperature_fact: temperature_fact,
        humidity: humidity,
        wind: wind,
        temperature_like: temperature_like,
    };

    data_weather.new_json();
}

fn temp_feels_like(temperature_fact: f32, humidity: f32, wind: f32) -> f32 {
    // Давление водяного пара, где humidity - влажность в процентах, а temperature_fact - температура в градусах
    let e =
        humidity / 100.0 * 6.105 * f32::exp(17.27 * temperature_fact / (237.7 + temperature_fact));
    let temp_like = temperature_fact + 0.33 * e - 0.7 * wind - 4.0;
    temp_like
}
