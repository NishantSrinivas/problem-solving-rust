/*
    Given a string returns the length of the last word of the string.

    Example
    input  : "hello world"
    output : 5

    input  : "bat man"
    output : 3

    input  : "good bye take care"
    output : 4
*/

use std::convert::TryInto;

fn length_of_last_word(s: String) -> i32 {
    let mut words: Vec<String> = Vec::new();
    let mut word: String = String::new();
    let s = s.trim();

    for (indx, i) in s.chars().enumerate() {
        if i != ' ' {
            word.push_str(&i.to_string());
        }

        if i == ' ' || indx == s.len() - 1 {
            if word != "" {
                words.push(word);
                word = String::new();
            }
        }
    }

    words[words.len() - 1].len().try_into().unwrap()
}

fn main() {
    let result: i32 = length_of_last_word(" hello good     lord world bye takecare".to_string());
    println!("{}", result);
}
