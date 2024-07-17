pub fn calc_alc_level(alchohol_consumed:f64,weight:f64,gender_ratio:f64,hours:f64)->f64{
    ((alchohol_consumed*5.14)/(weight*gender_ratio)) - (0.15*hours)
}