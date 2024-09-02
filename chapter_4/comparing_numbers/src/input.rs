use std::io;
pub fn get_num(prompt:&str)->i32{
    let mut buf = String::new();
    loop{
        println!("{prompt}");
        io::stdin().read_line(&mut buf).unwrap();
        let result = buf.trim().parse::<i32>();
        match result{
            Ok(num)=>{
                return num;
            },
            Err(__)=>{
                println!("Please input a number");
                buf.clear();
            }
        }
    }
    
}