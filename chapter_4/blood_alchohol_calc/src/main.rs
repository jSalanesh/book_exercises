mod input;
mod calc;
fn main() {
    //Calculates the blood alcohol level of a person
    //Prompt for gender
    //Prompt for weight in pounds
    //Prompt for alchohol in ounces
    //Prompt for number of hours since last drink
    let gender_ratio = input::get_gender("What is your gender? Press [M] for male and [F] for female");
    let weight = input::get_num("What is your weight in pounds");
    let alchohol_consumed = input::get_num("How much did you drink? In ounces");
    let hours = input::get_num("How many hours has it been since you last drank?");

    //Display whether safe to drive by comparing to 0.08
    let bac = calc::calc_alc_level(alchohol_consumed, weight, gender_ratio, hours);
    println!("Your blood alcohol level is {:.2}",bac);
    if bac > 0.08 {
        print!("You cannot drive.");
    }else{
        println!("You may safely drive.");
    }

    
}
