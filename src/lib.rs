pub mod template;

// Use this file to add helper functions and additional modules.

pub fn get_nums_usize(s: &str) -> Vec<usize> {
    let mut nums: Vec<usize> = vec![];
    let mut num: usize = 0;
    let mut num_found = false;
    for c in s.chars() {
        if c.is_ascii_digit() {
            num = 10 * num + (c.to_digit(10).unwrap() as usize);
            num_found = true;
        } else if num_found {
            nums.push(num);
            num  = 0;
            num_found = false;
        }
    }
    if num_found {
        nums.push(num);
    }
    nums
}