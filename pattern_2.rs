/*
N = 4

// Output
1
1 2
1 2 3 
1 2 3 4
*/

use std::io;
use std::process::exit;

fn main()
{
    let mut num_str: String = String::new();

    io::stdin().read_line(&mut num_str).expect("Couldn't read line");

    let num: u32 = match num_str.trim().parse()
    {
        Ok(val) => val,
        Err(e) => {println!("error: {}",e); exit(-1);}  
    };

    for i in 1..=num
    {
        let mut x: u32 = 1;
        while x <= i
        {
            print!("{} ",x);
            x+=1;
        }
        println!("");        
    }
}