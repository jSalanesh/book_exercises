use std::io;
pub fn get_num(prompt:&str)->f64{
    let mut buffer = String::new();
    loop{
        println!("{prompt}");
        io::stdin().read_line(&mut buffer).expect("Error reading line");
        let result = buffer.trim().parse::<f64>();
        match result{
            Ok(num)=>{
                return num;
            },
            Err(__)=>{
                println!("Please input a number");
                buffer.clear();
            }
        }
    }
}
pub enum Degree{
    Celsius,
    Fahrenheit,
    Kelvin,
}


pub fn get_choice(prompt:&str)->Degree{
    let mut buffer = String::new();
    loop{
        println!("{prompt}");
        io::stdin().read_line(&mut buffer).expect("Error reading line");
        let result = parse_choice(buffer.trim());
        match result {
            Ok(choice)=>{
                return choice;
            },
            Err(e)=>{
                println!("{e}");
                buffer.clear();
            }
        }
    }
}

fn parse_choice(choice:&str)->Result<Degree,&str>{
    match choice{
        "C" | "c" => Ok(Degree::Celsius),
        "F" | "f" => Ok(Degree::Fahrenheit),
        "K" | "k" => Ok(Degree::Kelvin),
        __ => {
            Err("Not a valid choice.")
        }
    }
}
