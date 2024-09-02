mod input;
fn main() {
    // Prompt for 3 different numbers
    // If they are not different then panic
    // Display the biggest number
    println!("Hello, world!");

    let num1 = input::get_num("Enter the first number:");
    let num2 = input::get_num("Enter the second number:");
    let num3 = input::get_num("Enter the third number:");

    let mut biggest_num:i32 = num1;
    if biggest_num < num2 {
        biggest_num = num2;
    }
    if biggest_num < num3 {
        biggest_num = num3;
    }
    
    println!("The biggest number is {}", biggest_num);

}
