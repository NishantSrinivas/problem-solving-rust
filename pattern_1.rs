/*
N = 4

//Output
* * * 
* * * 
* * * 
* * * 
*/

use std::io;
use std::process::exit;

fn main() {
    let mut num_str: String = String::new();

    io::stdin()
        .read_line(&mut num_str)
        .expect("couldn't read input");

    let num: u64 = match num_str.trim().parse() {
        Ok(val) => val,
        Err(_) => exit(-1),
    };

    for _ in 0..num
    {
        for i in 0..num-1
        {
            if i == num-2
            {
                print!("*");
            }
            else
            {
                print!("* ");
            }
        }
        println!("");
    }
}