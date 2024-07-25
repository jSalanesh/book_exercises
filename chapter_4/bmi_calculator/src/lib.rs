pub fn get_bmi(height:f64,weight:f64)->f64{
    (weight/(height*height))*703.0
}
#[cfg(test)]
mod tests{
use super::*;

#[test]
fn bmi_formula_test(){
    let result = get_bmi(65.0, 160.0);
    assert_eq!(result,26.6)
}
}