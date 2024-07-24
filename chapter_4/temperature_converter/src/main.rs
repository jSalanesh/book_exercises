use input::Degree;


mod input;

fn main() {
let first_choice = input::get_choice("What is the base temperature unit? [C]elsius, [F]ahrenheit, [K]elvin");
let second_choice = input::get_choice("Convert it into what temperature? [C]elsius, [F]ahrenheit, [K]elvin");
let current_temp = input::get_num("What is the temperature?");

match (first_choice,second_choice){
    (Degree::Celsius,Degree::Fahrenheit) => {
        println!("Converting from Celsius to Fahrenheit");
        println!("{}",temperature_converter::celsius_to_fahrenheit(current_temp));
    },
    (Degree::Celsius,Degree::Kelvin) => {
        println!("Converting from Celsius to Kelvin");
        println!("{}",temperature_converter::celsius_to_kelvin(current_temp));
    },
    (Degree::Fahrenheit,Degree::Celsius) => {
        println!("Converting from Fahrenheit to Celsius");
        println!("{}",temperature_converter::fahrenheit_to_celsius(current_temp));
    },
    (Degree::Fahrenheit,Degree::Kelvin) => {
        println!("Converting from Fahrenheit to Kelvin");
        println!("{}",temperature_converter::fahrenheit_to_kelvin(current_temp));
    },
    (Degree::Kelvin, Degree::Celsius) => {
        println!("Converting from Kelvin to Celsius");
        println!("{}",temperature_converter::kelvin_to_celsius(current_temp));
    },
    (Degree::Kelvin, Degree::Fahrenheit) => {
        println!("Converting from Kelvin to Fahrenheit");
        println!("{}",temperature_converter::kelvin_to_fahrenheit(current_temp));
    },
    __ => {
        println!("converting from same temperature");
    }
}
}
