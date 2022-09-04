/*
    Given a string s, check if it can be constructed by taking a substring of it and appending multiple copies of the substring together.

    Example
    input  : "abab"
    output : true

    input  : "aba"
    output : false
*/

fn repeated_substring_pattern(s: String) -> bool {
    let mut sub_str_len: usize = 1;
    let s_len: usize = s.len();

    while sub_str_len <= s_len {
        if s_len % sub_str_len == 0 {
            let mut pos: usize = 0;
            let mut prev_sub = &s[pos..pos + sub_str_len];

            while pos + sub_str_len <= s_len && prev_sub.len() != s_len {
                if prev_sub == &s[pos..pos + sub_str_len] {
                    prev_sub = &s[pos..pos + sub_str_len];

                    if pos + sub_str_len == s_len {
                        return true;
                    }
                } else {
                    break;
                }
                pos += sub_str_len;
            }
        }
        sub_str_len += 1;
    }

    false
}

fn main() {
    let result: bool;
    result = repeated_substring_pattern("abab".to_string());
    // result = repeated_substring_pattern("aba".to_string());
    // result = repeated_substring_pattern("abcabcabcabc".to_string());

    println!("result {}", result);
}
