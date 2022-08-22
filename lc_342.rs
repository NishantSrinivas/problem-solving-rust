/*
    Prints true if a number is a power of 4.

    Example
    input  : 16
    output : True

    input  : 5
    output : False

    input  : 1
    output : True
*/

use std::io;
use std::process::exit;

fn main() {
    let mut str_inp: String = String::new();

    io::stdin()
        .read_line(&mut str_inp)
        .expect("Couldn't read input");

    let int_inp: i32 = match str_inp.trim().parse()
    {
        Ok(val) => val,
        Err(_) =>  exit(-1)
    };

    let mut val: i32 = int_inp.abs().clone();
    let mut pow = 0;
    while val > 1
    {
        if val % 4 != 0
        {
            println!("{}",false);
            break;
        }
        val = val / 4;
        pow += 1;
    }

    if 4_i32.pow(pow) == int_inp
    {
        println!("{}",true);
    }
    else
    {
        println!("{}",false);
    }
}
