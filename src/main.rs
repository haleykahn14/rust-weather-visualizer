use std::io;
use nannou::prelude::*;
use reqwest::blocking::Client;
use serde_json::Value;

struct Model {
    texture: wgpu::Texture,
}

fn main() {
    
    nannou::app(model).run();
}

    

// Asks the user to input the name of a city
fn get_city()-> String {
    println!("Enter the name of a city you would like the weather for:");
    let mut city = String::new();
    io::stdin().read_line(&mut city).expect("Failed to read line");
    city = city.trim().to_string();
    println!("The city you entered is: {}", city);
    city
}

fn get_city_filepath(city: String) -> String {
    let mut filepath = "";

    let fix_city = city.to_lowercase();
    if fix_city == "kyoto" {
        filepath =  "src/assets/kyoto.png";
    } else if fix_city == "tokyo" {
        filepath =  "src/assets/tokyo.png";
    } else if fix_city == "london" {
        filepath =  "src/assets/london.png";
    } else if fix_city == "madrid" {
        filepath =  "src/assets/madrid.webp";
    } else if fix_city == "nashville" {
        filepath =  "src/assets/nashville.png";
    } else if fix_city == "new york" {
        filepath =  "src/assets/newyork.png";
    } else {
        filepath =  "src/assets/Empty.png";
    }

    println!("The filepath is: {}", filepath);
    return filepath.to_string();
}

//gets the weather for the inputted city
fn get_weather(city: String) -> (f64, String) {
    let api_key = "821c7713a2d08ad1cab07370fa82cfb7";
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", city, api_key);

    let client = Client::new();
    let response = client.get(&url).send().unwrap();

    if response.status().is_success() {
    let json: Value = response.json().unwrap();
    let temperature = json["main"]["temp"].as_f64().unwrap();
    let weather = json["weather"][0]["description"].as_str().unwrap().to_string();

    return (temperature, weather)

    } else {

    return (0 as f64, format!("Error: {}", response.status()))
    }
}


//creates the model
fn model(app: &App) -> Model {
    let city = get_city();
    let filepath = get_city_filepath(city);

    // Create a new window!
    app.new_window()
        .size(512, 512)
        .view(view)
        .build()
        .unwrap();

    let texture = wgpu::Texture::from_path(app, filepath).unwrap();
    Model { texture }
}

//renders the visualization
fn view(app: &App, model: &Model, frame: Frame) {
    // let weather_report = get_weather(city);

    frame.clear(BLACK);

    let draw = app.draw();
    draw.texture(&model.texture);
    draw.background().color(PLUM);

    draw.to_frame(app, &frame).unwrap();
}












