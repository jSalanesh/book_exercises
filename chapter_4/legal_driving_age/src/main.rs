mod input;
fn main() {
    let age = input::get_age("What is your age?");
    let mut output_statement = String::new();
    if age < 18 {
        output_statement = String::from("You are not legally allowed to drive.");
    }else{
        output_statement = String::from("You are old enough to drive.");
    }
    println!("{}",output_statement);
}
