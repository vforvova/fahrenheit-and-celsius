fn main() {
    use std::env;

    let fahrenheit_deg = env::args().nth(1).unwrap().parse::<f32>().unwrap();
    let celsius_deg = convert(fahrenheit_deg);
    println!("{} degrees in Fahrenheit are equal to {} degrees in Celsius", fahrenheit_deg, celsius_deg);
}

fn convert(deg: f32) -> f32 {
    (deg - 32.0) * 5.0 / 9.0
}
