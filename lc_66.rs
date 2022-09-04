/*
    You are given a large integer represented as an integer array digits, where each digits[i] is the ith digit of the integer. The digits are ordered from most significant to least significant in left-to-right order. The large integer does not contain any leading 0's.
    Increment the large integer by one and return the resulting array of digits.

    Example
    input:  [1, 2, 3]
    output: [1, 2, 4]

    input:  [9, 9, 9]
    output: [1, 0, 0, 0]
*/

fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut v: Vec<i32> = digits;
    let mut pos: usize = v.len() - 1;

    let mut result: i32 = v[pos] + 1;

    if result < 10 {
        v[pos] = result;
        return v;
    }

    while result > 9 && pos > 0 {
        v[pos] = 0;
        pos -= 1;
        v[pos] += 1;
        result = v[pos];
    }

    if pos == 0 && result > 9 {
        v[pos] = 0;
        v.insert(pos, 1);
    }

    v
}

fn main() {
    println!("{:?}", plus_one(vec![1, 2, 3, 9, 9]));
    // println!("{:?}", plus_one(vec![1]));
    // println!("{:?}", plus_one(vec![3, 9, 9]));
    // println!("{:?}", plus_one(vec![9, 9, 9]));
    // println!("{:?}", plus_one(vec![1, 9, 9, 9]));
    // println!("{:?}", plus_one(vec![9]));
    // println!("{:?}", plus_one(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]));
    // println!("{:?}", plus_one(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 9]));
    // println!(
    //     "{:?}",
    //     plus_one(vec![
    //         7, 2, 8, 5, 0, 9, 1, 2, 9, 5, 3, 6, 6, 7, 3, 2, 8, 4, 3, 7, 9, 5, 7, 7, 4, 7, 4, 9, 4,
    //         7, 0, 1, 1, 1, 7, 4, 0, 0, 6
    //     ])
    // );
}
