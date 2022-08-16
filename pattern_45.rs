/*
N = 4

// Output
        1
      1 2 1
    1 2 3 2 1
  1 2 3 4 3 2 1
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

    for i in 1..=num
    {
        let mut x:u64 = 1;
        for _ in 0..num-i
        {
            print!("  ");
        }
        while x <= i
        {
            print!("{} ",x);
            x+=1;
        }
        x -= 2;
        while x >= 1
        {
            print!("{} ",x);
            x-=1;
        }
        println!("");
    } 
}
