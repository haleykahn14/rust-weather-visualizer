use std::io;
use nannou::{draw, prelude::*};
use reqwest::blocking::Client;
use serde_json::Value;

struct Model {
    texture: wgpu::Texture,
    city: ((f64, i64), String),
}

fn main() {
    println!("****** Welcome to Haley's Weather Visualization App! ******");
    println!("This application provides real time visualization of the weather in a city of your choice.");
    println!("The visualization will be displayed in a window and will include a representation of the weather conditions in the city and the temperature.");
    println!("");
    nannou::app(model).run();
}

    

// Asks the user to input the name of a city
fn get_city()-> String {
    println!("Enter the name of a city you would like the weather for:");
    let mut city = String::new();
    io::stdin().read_line(&mut city).expect("Failed to read line");
    city = city.trim().to_string();
    city
}

fn get_city_filepath(city: &String) -> String {
    let mut filepath = "";

    let fix_city = city.to_lowercase();
    if fix_city == "kyoto" {
        filepath =  "src/assets/kyoto.png";
    } else if fix_city == "tokyo" {
        filepath =  "src/assets/tokyo.png";
    } else if fix_city == "london" {
        filepath =  "src/assets/london.png";
    } else if fix_city == "madrid" {
        filepath =  "src/assets/madrid.png";
    } else if fix_city == "nashville" {
        filepath =  "src/assets/nashville.png";
    } else if fix_city == "new york" {
        filepath =  "src/assets/newyork.png";
    } else {
        filepath =  "src/assets/Empty.png";
    }

    return filepath.to_string();
}

//gets the weather for the inputted city
fn get_weather(city: &String) -> ((f64, i64), String) {
    let api_key = "821c7713a2d08ad1cab07370fa82cfb7";
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", city, api_key);

    let client = Client::new();
    let response = client.get(&url).send().unwrap();

    if response.status().is_success() {
    let json: Value = response.json().unwrap();
    let temperature = json["main"]["temp"].as_f64().unwrap();
    let weather = json["weather"][0]["description"].as_str().unwrap().to_string();
    let weather_id = json["weather"][0]["id"].as_i64().unwrap();
    let city_name_fixed = json["name"].as_str().unwrap().to_string();
    println!("The temperature in {} is {} degrees Celsius and the forecast is: {}", city_name_fixed, temperature, weather);

    return ((temperature, weather_id), weather);

    } else {
        ((0.0, 0), "No weather data available".to_string())
    }
}


//creates the model
fn model(app: &App) -> Model {
    let my_city = get_city();
    let filepath = get_city_filepath(&my_city);

    // Create a new window!
    app.new_window()
        .size(1024, 512)
        .view(view)
        .build()
        .unwrap();

    let my_texture = wgpu::Texture::from_path(app, filepath).unwrap();
    let weather = get_weather(&my_city);

    Model { 
        texture: my_texture, 
        city: weather,
    }
}

//renders the visualization
fn view(app: &App, model: &Model, frame: Frame) {

    let (temperature, weather) = &model.city.0;
    let my_temp = get_temp_color(temperature);

    
    let draw = app.draw();
    draw.texture(&model.texture);
    draw.background().color(my_temp);
    
    

    // light thunderstorms
    if weather == &(200 as i64) || weather == &(201 as i64) ||  weather == &(210 as i64) || weather == &(230 as i64) || weather == &(231 as i64) || weather == &(232 as i64){
        draw_thunderstorm(model, app, my_temp, 50);
    }
    // heavy thunderstorms
    else if weather == &(202 as i64) || weather == &(211 as i64) || weather == &(212 as i64) || weather == &(221 as i64){
        draw_thunderstorm(model, app, my_temp, 100);
    }
    // drizzle
    else if weather == &(300 as i64) || weather == &(301 as i64) || weather == &(302 as i64) || weather == &(310 as i64) || weather == &(311 as i64) || weather == &(312 as i64) || weather == &(313 as i64) || weather == &(314 as i64) || weather == &(321 as i64){
        draw_rain(model, app, my_temp, 10);
    }
    // light to medium rain
    else if weather == &(500 as i64) || weather == &(501 as i64) || weather == &(520 as i64) || weather == &(521 as i64) || weather == &(531 as i64) || weather == &(511 as i64){
        draw_rain(model, app, my_temp, 50);
    }
    // heavy rain
    else if weather == &(502 as i64) || weather == &(503 as i64) || weather == &(504 as i64) || weather == &(522 as i64){
        draw_rain(model, app, my_temp, 100);
    }
    // light snow
    else if weather == &(600 as i64) || weather == &(601 as i64) || weather == &(612 as i64) || weather == &(615 as i64) || weather == &(616 as i64) || weather == &(620 as i64) || weather == &(621 as i64) || weather == &(622 as i64){
        draw_snow(model, app, my_temp);
    }
    // heavy snow
    else if weather == &(602 as i64) || weather == &(622 as i64){
        draw_snow(model, app, my_temp);
    }
    // sleet
    else if weather == &(611 as i64) || weather == &(612 as i64) || weather == &(613 as i64){
        draw_sleet(model, app, my_temp);
    }
    // mist and haze and fog
    else if weather == &(701 as i64) ||  weather == &(721 as i64){
        draw_atmospheric_particles(model, app, my_temp, LIGHTGRAY);
    }
    // smoke
    else if weather == &(711 as i64){
        draw_atmospheric_particles(model, app, my_temp, DARKGRAY);
    }
    // dust
    else if weather == &(731 as i64) || weather == &(761 as i64) {
        draw_atmospheric_particles(model, app, my_temp, BURLYWOOD);
    }
    // sand
    else if weather == &(751 as i64){
        draw_atmospheric_particles(model, app, my_temp, SANDYBROWN);
    }
    // ash
    else if weather == &(762 as i64){
        draw_atmospheric_particles(model, app, my_temp, GRAY);
    }
    // squalls
    else if weather == &(771 as i64){
        draw_squalls(model, app, my_temp);
    }
    // tornado
    else if weather == &(781 as i64){
        draw_tornado(model, app, my_temp);
    }
    // clear sky
    else if weather == &(800 as i64){
        draw_clear_sky(model, app, my_temp);
    }
    // few clouds
    else if weather == &(801 as i64){
        draw_overcast(model, app, my_temp, 10, false);
    }
    // scattered clouds
    else if weather == &(802 as i64){
        draw_overcast(model, app, my_temp, 50, false);
    }
    // broken clouds
    else if weather == &(803 as i64){
        draw_overcast(model, app, my_temp, 75, false);
    }
    // overcast clouds
    else if weather == &(804 as i64){
        draw_overcast(model, app, my_temp, 100, false);
    }
    else {
        return;
    }



    draw.to_frame(app, &frame).unwrap();
}

fn get_temp_color(temperature: &f64) -> Srgb<u8> {

    let my_temp;

    if temperature > &46.0{
        my_temp = BLACK;
    } else if temperature > &38.0 {
        my_temp = DARKRED;
    } else if temperature > &29.0 {
        my_temp = CRIMSON;
    }
    else if temperature > &24.0 {
        my_temp = ORANGERED;
    }
    else if temperature > &16.0 {
        my_temp = ORANGE;
    }
    else if temperature > &10.0 {
        my_temp = GOLD;
    }
    else if temperature > &4.0{
        my_temp = LIGHTYELLOW;
    }
    else if temperature > &-1.0{
        my_temp = PALEGREEN;
    }
    else if temperature > &-9.0{
        my_temp = POWDERBLUE;
    }
    else if temperature > &-18.0{
        my_temp = ROYALBLUE;
    }
    else if temperature > &-23.0{
        my_temp = SLATEBLUE;
    }
    else if temperature >= &-29.0{
        my_temp = REBECCAPURPLE;
    }
    else {
        my_temp = INDIGO;
    }

    my_temp
}

fn draw_weather_label(model: &Model, app: &App, temp: Srgb<u8>) {
    let draw = app.draw();
    draw.texture(&model.texture);
    draw.background().color(temp);
    let win = app.window_rect();

    let forecast = format!("Forecast: {}", &model.city.1);
    let forecast_str: &str = &forecast;

    draw.text(forecast_str)
        .x_y(-300.0, win.top() - 400.0)
        .color(BLACK)
        .font_size(24);


    let number_string: String = (&model.city.0.0).to_string();
    let number_str: &str = &number_string;
    let temp_str = format!("Temperature: {} °C", number_str);

    draw.text(&temp_str)
    .x_y(-300.0, win.top() - 460.0)
    .color(BLACK)
    .font_size(24);

}


fn draw_rain(model: &Model, app: &App, temp: Srgb<u8>, speed: i32) {
    let draw = app.draw();
    draw.texture(&model.texture);
    draw.background().color(temp);

    draw_overcast(model, app, temp, speed, true);
    
    let win = app.window_rect();
    let n_drops = speed;
    for _ in 0..n_drops {
        let x = random_range(win.left(), win.right());
        let y = random_range(win.top()-200.0, win.bottom());
        draw.ellipse().xy(pt2(x, y)).radius(10.0).color(BLUE);
    }

}



fn draw_thunderstorm(model: &Model, app: &App, temp: Srgb<u8>, speed: i32) {
    let draw = app.draw();
    draw.texture(&model.texture);
    draw.background().color(temp);

    let win = app.window_rect();

    draw_overcast(model, app, temp, speed, true);
    draw_rain(model, app, temp, speed);

    for _ in 0..10 {
        let start_x = random_range(win.left(), win.right());
        let start_y = random_range(win.top(), win.bottom());
        let end_x = random_range(win.left(), win.right());
        let end_y = random_range(win.top(), win.bottom());
        draw.polyline()
            .weight(2.0)
            .points(vec![pt2(start_x, start_y), pt2(end_x, end_y)])
            .color(YELLOW);
        
    }

}


fn draw_snow(model: &Model, app: &App, temp: Srgb<u8>) {
    let draw = app.draw();
    draw.texture(&model.texture);
    draw.background().color(temp);

    draw_weather_label(model, app, temp);

    let win = app.window_rect();
    let n_drops = 5;
    for _ in 0..n_drops {
        let x = random_range(win.left(), win.right());
        let y = random_range(win.top()-200.0, win.bottom());
        draw.ellipse().xy(pt2(x, y)).radius(10.0).color(WHITE);
    }

}

fn draw_sleet(model: &Model, app: &App, temp: Srgb<u8>) {
    let draw = app.draw();
    draw.texture(&model.texture);
    draw.background().color(temp);

    draw_rain(model, app, temp, 10);

    let win = app.window_rect();
    let n_drops = 100;
    for _ in 0..n_drops {
        let x = random_range(win.left(), win.right());
        let y = random_range(win.top()-200.0, win.bottom());
        draw.ellipse().xy(pt2(x, y)).radius(10.0).color(WHITE);
    }

}


fn draw_overcast(model: &Model, app: &App, temp: Srgb<u8>, speed: i32, rain: bool) {
    let draw = app.draw();
    draw.texture(&model.texture);
    draw.background().color(temp);

    let win = app.window_rect();

    draw_weather_label(model, app, temp);

    let cloud_color;
    if rain {
        cloud_color = DIMGRAY;
    } else {
        cloud_color = LIGHTGRAY;
        if speed < 50 {
            draw_clear_sky(model, app, temp);
        }
    }
    

    let n_clouds = speed;
    for _ in 0..n_clouds {
        let x = random_range(win.left(), win.right());
        let y = random_range(win.top(), win.bottom()+300.0);
        draw.ellipse().color(cloud_color).w(90.0).h(60.0).x_y(x, y);
        draw.ellipse().color(cloud_color).w(90.0).h(60.0).x_y(x, y+50.0);
        draw.ellipse().color(cloud_color).w(90.0).h(60.0).x_y(x-50.0, y);
        draw.ellipse().color(cloud_color).w(90.0).h(60.0).x_y(x+-0.0, y+25.0);
        draw.ellipse().color(cloud_color).w(90.0).h(60.0).x_y(x+50.0, y+25.0);

    }


}

fn draw_atmospheric_particles(model: &Model, app: &App, temp: Srgb<u8>, weather_cond: Srgb<u8>) {
    let draw = app.draw();
    draw.texture(&model.texture);
    draw.background().color(temp);

    draw_weather_label(model, app, temp);

    let win = app.window_rect();
    let n_drops = 2000;
    for _ in 0..n_drops {
        let x = random_range(win.left(), win.right());
        let y = random_range(win.top(), win.bottom());
        draw.ellipse().xy(pt2(x, y)).radius(1.0).color(weather_cond);
    }

}

fn draw_squalls(model: &Model, app: &App, temp: Srgb<u8>) {
    let draw = app.draw();
    draw.texture(&model.texture);
    draw.background().color(temp);

    let win = app.window_rect();
    draw_weather_label(model, app, temp);

    // Draw wind lines
    let n_lines = 50;
    for _ in 0..n_lines {
        let start_x = random_range(win.left(), win.right());
        let start_y = random_range(win.top(), win.bottom());
        let end_x = start_x + random_range(50.0, 150.0);
        let end_y = start_y + random_range(-20.0, 20.0);
        draw.line()
            .start(pt2(start_x, start_y))
            .end(pt2(end_x, end_y))
            .weight(2.0)
            .color(GAINSBORO);
    }

}

fn draw_tornado(model: &Model, app: &App, temp: Srgb<u8>) {
    let draw = app.draw();
    draw.texture(&model.texture);
    draw.background().color(temp);

    let win = app.window_rect();

    draw_squalls(model, app, temp);
    

    // Draw the funnel shape of the tornado
    let funnel_height = 300.0;
    let funnel_width = 200.0;
    let funnel_steps = 50;
    let step_height = funnel_height / funnel_steps as f32;
    let step_width = funnel_width / funnel_steps as f32;

    for i in 0..funnel_steps {
        let y = (win.top()-100.0) - i as f32 * step_height;
        let width = funnel_width - i as f32 * step_width;
        draw.ellipse()
            .x_y(0.0, y)
            .w_h(width, step_height)
            .color(DIMGRAY)
            .stroke(BLACK)
            .stroke_weight(1.0);
    }

}

fn draw_clear_sky(model: &Model, app: &App, temp: Srgb<u8>) {
    let draw = app.draw();
    draw.texture(&model.texture);
    draw.background().color(temp);

    let win = app.window_rect();

    draw_weather_label(model, app, temp);

    // Draw the sun
    let sun_radius = 50.0;
    draw.ellipse()
        .x_y(0.0, win.top() - sun_radius - 50.0)
        .w_h(sun_radius * 2.0, sun_radius * 2.0)
        .color(YELLOW);

    // Draw sun rays
    let n_rays = 20;
    let ray_length = 100.0;
    for i in 0..n_rays {
        let angle = i as f32 * (360.0 / n_rays as f32);
        let (sin, cos) = angle.to_radians().sin_cos();
        let start_x = cos * sun_radius;
        let start_y = win.top() - sun_radius - 50.0 + sin * sun_radius;
        let end_x = cos * (sun_radius + ray_length);
        let end_y = win.top() - sun_radius - 50.0 + sin * (sun_radius + ray_length);
        draw.line()
            .start(pt2(start_x, start_y))
            .end(pt2(end_x, end_y))
            .weight(2.0)
            .color(YELLOW);
    }

}





