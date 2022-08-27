/*
    Finds the longest common prefix of N number of words.

    Example
    input  : ["flower", "flow", "flight"]
    output : fl

    input  : ["orange", "mango", "banana"]
    output : ""

*/

fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 0 {
        return "".to_string().clone();
    }

    if strs.len() == 1 {
        return strs[0].clone();
    }

    let mut s = String::from("");

    let mut smallest_size: usize = 201;
    let mut smallest_index: usize = 201;

    let mut len: usize;
    for (idx, i) in strs.iter().enumerate() {
        len = i.len();
        if len < smallest_size {
            smallest_size = len;
            smallest_index = idx;
        }
    }

    for (indx, chrs) in strs[smallest_index].chars().enumerate() {
        let mut flag: bool = true;

        for word in strs.iter() {
            if chrs != word.as_bytes()[indx] as char {
                flag = false;
                break;
            }
        }

        if flag {
            s.push_str(&chrs.to_string());
        } else {
            break;
        }
    }
    s
}

fn main() {
    // let strs: Vec<String> = Vec::from(["helloworld".to_string(),"helloworldlenny".to_string(),"helloworldbarny".to_string()]);
    // let strs: Vec<String> = Vec::from(["hello".to_string(),"helloworldlenny".to_string(),"helloworld".to_string()]);
    // let strs: Vec<String> = Vec::from(["helloworld".to_string()]);
    // let strs: Vec<String> = Vec::from(["apple".to_string(),"banana".to_string(),"orange".to_string()]);
    // let strs: Vec<String> = Vec::new();
    // let strs: Vec<String> = Vec::from(["ha".to_string(),"hb".to_string(),"hc".to_string()]);
    // let strs: Vec<String> = Vec::from(["h".to_string(),"h".to_string(),"h".to_string()]);
    // let strs: Vec<String> = Vec::from([
    //     "flower".to_string(),
    //     "flow".to_string(),
    //     "flight".to_string(),
    // ]);
    println!("{}", longest_common_prefix(strs));
}
