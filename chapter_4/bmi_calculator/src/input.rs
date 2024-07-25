use std::io;
pub fn get_num(prompt:&str)->f64{
  let mut buf = String::new();
  loop{
    println!("{prompt}");
    io::stdin().read_line(&mut buf).unwrap();
    let result = buf.trim().parse::<f64>();
    match result{
        Ok(num)=>{
            return num;
        },
        Err(__)=>{
            println!("Please input a number");
        }
    }
  }
}