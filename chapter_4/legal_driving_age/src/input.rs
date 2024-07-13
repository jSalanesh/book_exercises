use std::io;
pub fn get_age(prompt:&str)->i32{
    let mut buffer = String::new();
    loop{     
        println!("{prompt}");
        io::stdin().read_line(&mut buffer).unwrap();
        let result = buffer.trim().parse::<i32>();
        match result {
            Ok(num) => {
                return num;
            },
            Err(__) => {
                println!("Please enter a number");
                buffer.clear();
            }
        }
    }
}