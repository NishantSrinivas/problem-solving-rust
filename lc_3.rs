/*
    Prints the length of the longest substring.

    Example
    input  : "captainamerica"
    output : 7

    input  : "abba"
    output : 2

    input  : "wintersoldier"
    output : 10
*/

use std::collections::HashMap;
use std::convert::TryInto;

fn length_of_longest_substring(s: String) -> i32 {
    if s.len() == 0 {
        return 0;
    }

    let mut final_ans: i32 = 0;
    let mut curr_ans: i32;

    let mut start_index: i32 = 0;
    let mut end_index: i32;

    let mut char_last_seen: HashMap<char, i32> = HashMap::<char, i32>::new();

    for (indx, chr) in s.chars().enumerate() {
        if !char_last_seen.contains_key(&chr) {
            char_last_seen.insert(chr, indx.try_into().unwrap());
            end_index = indx.try_into().unwrap();
        } else {
            if char_last_seen[&chr] >= start_index {
                start_index = char_last_seen[&chr] + 1;
            }
            char_last_seen.insert(chr, indx.try_into().unwrap());
            end_index = indx.try_into().unwrap();
        }

        curr_ans = end_index - start_index;
        if curr_ans > final_ans {
            final_ans = curr_ans;
        }
    }

    final_ans + 1
}

fn main() {
    let mut result: i32;

    let words: Vec<&str> = Vec::from([
        "hello",
        "abcabcbb",
        "bbbbb",
        "pwwkew",
        "ababcabcde",
        "ababcdef",
        "magic",
        "butterfly",
        "desert",
        "icelake",
        "cometlake",
        "raptorlake",
        "diamond",
        "abba",
        "abiba",
        "",
        "l",
        "icici",
        "captainamerica",
        "wintersoldier",
        "moonknight",
        "shangchi",
        "essexserpent",
        "magma"
    ]);

    for word in words {
        result = length_of_longest_substring(word.to_string());
        println!("word : {} | ans : {}", word, result);
    }
}
