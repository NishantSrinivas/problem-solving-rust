/*
    Finds the fist non repeating character in a string,
    and returns it's index. If no such character exists
    return -1.

    Example
    input  : hello
    output : 0

    input  : bulb
    output : 1

    input  : aabb
    output : -1
*/

use std::io;
use std::collections::HashMap;
use std::convert::TryInto;

fn main()
{
    let mut inp_str: String = String::new();
    let mut char_map: HashMap::<char, i32> = HashMap::<char, i32>::new();

    io::stdin().read_line(&mut inp_str).expect("Couldn't read user input");

    if inp_str.trim().len() == 1
    {
        println!("0");
        return;
    }

    for (i,x) in inp_str.trim().chars().enumerate()
    {
        if !char_map.contains_key(&x)
        {
            char_map.insert(x,i.try_into().unwrap());
        }
        else
        {
            char_map.insert(x,-1);
        }
    }

    if char_map.is_empty()
    {
        println!("-1");
        return;
    }
    else
    {
        let mut min: i32 = i32::MAX;
        let mut f: bool = false;
        for idx in char_map.values()
        {
            if idx >= &0 && idx <= &min
            {
                min = *idx;
                f = true;
            }
        }

        if !f
        {
            println!("-1");
        }
        else
        {
            println!("{}",min);
        }
    }

    // debug op
    // println!("{:?}",char_map);
    // println!("{:?}",char_map.values());
}
