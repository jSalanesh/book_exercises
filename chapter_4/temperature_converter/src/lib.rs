pub fn celsius_to_fahrenheit(temp:f64)->f64{
    (temp*1.8)+32.0
}

pub fn celsius_to_kelvin(temp:f64)->f64{
    temp + 273.15
}

pub fn fahrenheit_to_celsius(temp:f64)->f64{
    (temp-32.0)*(5.0/9.0)
}

pub fn fahrenheit_to_kelvin(temp:f64)->f64{
    (temp-32.0)*(5.0/9.0)+273.15
}

pub fn kelvin_to_celsius(temp:f64)->f64{
    temp - 273.15
}

pub fn kelvin_to_fahrenheit(temp:f64)->f64{
    (temp - 273.15)*(9.0*5.0)+32.0
}




