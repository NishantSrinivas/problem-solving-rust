use std::io;

fn main() {
    let mut user_input: String = String::new();

    // read line return number of characters read form stdin.
    println!("Enter some text");
    let temp = io::stdin().read_line(&mut user_input); 

    match temp {
        Ok(input) => println!("entered: {}",input),
        Err(e) => println!("Err: {}",e) 
    };

    println!("user_input: {}",user_input.trim());

    let mut inp_num:String = String::new();
    println!("Enter some int");

    io::stdin().read_line(&mut inp_num).expect("Failed to read the line");

    let num: u32 = match inp_num.trim().parse()
    {
        Ok(val) =>  val,
        Err(e) => {println!("{}",e); 0}
    };

    println!("number you entered {}",num);

    // println!("Hello form test file");
}
