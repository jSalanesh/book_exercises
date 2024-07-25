mod input;
fn main() {
    // Prompt for height in inches
    // Prompt for weight in pounds
    // Calculate BMI
    // Display BMI and whether they are underweight or overweight
    // 18.5 to 25 is ideal BMI
   let height = input::get_num("Give me your height in inches");
   let weight = input::get_num("What is your weight in pounds?");
   let bmi = bmi_calculator::get_bmi(height, weight);
   println!("You have a BMI of {:.2} ",bmi);
   
   if bmi < 18.5{
    println!("You are underweight, please see a doctor.");
   } else if bmi > 25.0 {
    println!("You are overweight, please see a doctor.");
   } else {
    println!("You are in the ideal weight range. Congratulations!");
   }

}
