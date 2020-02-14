use std::env;
use std::fs;
use std::path::Path;

fn read_ip7_list(filename: &str) -> String {
    let fpath = Path::new(filename);
    let abspath = env::current_dir().unwrap().into_boxed_path().join(fpath);
    let list = fs::read_to_string(abspath).expect("Error reading file");
    list
}

fn validate_ip7(ip7_list:&str)->i32{
    let mut count = 0;
    for ip7 in ip7_list.lines(){
        if !has_invalid_abba(ip7){
            if has_abba(ip7){
                count+=1;
            }
        }
    }
    count
}

fn has_invalid_abba(ip7: &str) -> bool {
    let open_brkts = find_all_indices(ip7, '[');
    let closed_brkts = find_all_indices(ip7, ']');

    for h in 0..open_brkts.len() {
        let hyp = &ip7[open_brkts[h] + 1..closed_brkts[h]];
        if has_abba(hyp) {
            return true;
        }
    }
    return false;
}

fn has_abba(ip7: &str) -> bool {
    for ch in ip7.chars() {
        if ip7.matches(ch).count() > 1 && ch != '[' && ch != ']' {
            let indices = find_all_indices(ip7, ch);
            for i in 1..indices.len() {
                if indices[i] - indices[i - 1] == 3 {
                    return has_matching_pair(ip7,ch, indices[i - 1], indices[i]);
                }
            }
        }
    }
    return false;
}

fn find_all_indices(src: &str, ch: char) -> Vec<usize> {
    let mut indices: Vec<usize> = Vec::new();
    for (i, c) in src.chars().enumerate() {
        if c == ch {
            indices.push(i)
        }
    }
    indices
}

fn has_matching_pair(src: &str, ch: char, left: usize, right: usize) -> bool {
    let src_str = src.as_bytes();
    if src_str[left + 1] == src_str[right - 1]
        && (right - left == 3)
        && src_str[left + 1] != ch as u8
        && src_str[right - 1] != ch as u8
    {
        return true;
    }
    return false;
}

pub fn run() {
    let ip7_list = read_ip7_list("inputs/day-7.txt");
    let count = validate_ip7(&ip7_list);
    println!("{}", count);

}
