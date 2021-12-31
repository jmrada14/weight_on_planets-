// Simple CLI tool to calculate your weight on different planets and moons.
use std::io;


fn main() {
    println!("Welcome to the weight on planets calculator!");
    println!("---------------------------------------------");
    println!("Enter a planet: ");
    let mut planet_input = String::new();
    io::stdin().read_line(&mut planet_input).unwrap();
    let planet: String = planet_input.to_lowercase().trim().parse().unwrap();
    println!("Enter your weight (kg): ");
    let mut weight_input = String::new();
    io::stdin().read_line(&mut weight_input).unwrap();
    let weight: f32 = weight_input.trim().parse().unwrap();

    println!("Your weight on {} is {} kg", planet, weight_on_planet(weight, &planet));
}

fn weight_on_planet(weight: f32, planet: &str) -> f32 {
    let mut weight_on_planet = weight;
    match planet {
        "earth" => weight_on_planet *= 1.0,
        "jupiter" => weight_on_planet *= 2.34,
        "mars" => weight_on_planet *= 0.38,
        "mercury" => weight_on_planet *= 0.38,
        "neptune" => weight_on_planet *= 1.12,
        "saturn" => weight_on_planet *= 0.93,
        "uranus" => weight_on_planet *= 0.92,
        "venus" => weight_on_planet *= 0.91,
        _ => println!("Unknown planet"),
    }
   return weight_on_planet;
}
