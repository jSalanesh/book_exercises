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
pub fn get_gender(prompt:&str)->f64{
    let mut buffer = String::new();
    loop{
        println!("{prompt}");
        io::stdin().read_line(&mut buffer).expect("Error reading line");
        let gender = buffer.trim().parse::<String>().unwrap();
        match gender.as_str(){
            "M" | "m"  => {
                return 0.73
            },
            "F"  | "f" => {
                return 0.66
            }
            _ => {
                println!("Not a valid gender. Press [M] or [F]");
                buffer.clear();
            }
        }
    }
}
