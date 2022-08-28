/*
    Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.

    Example
    input  : "captainamerica", "america"
    output : 7

    input  : "batman", "bat"
    output : 0

    input  : "batman", "man"
    output : 3
*/

use std::convert::TryInto;

fn str_str(haystack: String, needle: String) -> i32 {
    if needle.len() == 0 || haystack.len() == 0 {
        return 0;
    }

    if haystack.len() < needle.len() {
        return -1;
    }

    let mut i: usize;

    for indx in 0..haystack.len() {
        i = 0;

        if haystack.as_bytes()[indx] == needle.as_bytes()[i] {
            while i < needle.len()
                && indx + i < haystack.len()
                && haystack.as_bytes()[indx + i] == needle.as_bytes()[i]
            {
                i += 1;
            }
        }

        if i == needle.len() {
            return indx.try_into().unwrap();
        }
    }

    -1
}

fn main() {
    let index: i32;

    // index = str_str("batman".to_string(), "man".to_string());
    // index = str_str("batman".to_string(), "bat".to_string());
    // index = str_str("batman".to_string(), "cat".to_string());
    // index = str_str("batcatman".to_string(), "cat".to_string());
    // index = str_str("".to_string(), "cat".to_string());
    // index = str_str("batman".to_string(), "".to_string());
    // index = str_str("hello".to_string(), "ll".to_string());
    // index = str_str("aaaaa".to_string(), "bba".to_string());
    index = str_str("aaaaa".to_string(), "aaaaaaaaa".to_string());

    println!("{}", index);
}
