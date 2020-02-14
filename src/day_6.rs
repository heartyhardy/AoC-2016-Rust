use std::env;
use std::fs;
use std::path::Path;
use std::collections::HashMap;
//easter
#[derive(Copy, Clone, Debug,Eq,PartialEq)]
struct Charcount{
    ch:char,
    count:i32
}

fn read_repetition_messages(filename: &str) -> String {
    let fpath = Path::new(filename);
    let abspath = env::current_dir().unwrap().into_boxed_path().join(fpath);
    let codes = fs::read_to_string(abspath).expect("Error reading file!");
    codes
}

fn arrange(messages:&str)->Vec<Vec<char>>{
    let lines:Vec<&str> =messages.split("\n").collect();
    let mut cols:Vec<Vec<char>> = Vec::new();

    let mut i = 0;
    for line in lines{
        for (j,ch) in line.chars().enumerate(){
            if i<line.chars().count(){
                let mut col:Vec<char> = Vec::new();
                col.push(ch);
                cols.push(col);
                i+=1;
            }else{
                cols[j].push(ch);
            }
        }
    }
    cols
}

fn error_correction(colums:Vec<Vec<char>>){
    let mut corrected = String::new();
    let mut corrected_after_mod = String::new();
    for col in colums{
        let mut charmap:HashMap<char,Charcount> = HashMap::new();
        for ch in col{
            if charmap.contains_key(&ch){
                charmap.get_mut(&ch).unwrap().count+=1;
            }else{
                charmap.insert(ch, Charcount{ch:ch,count:1});
            }
        }
        let mut values:Vec<Charcount> = charmap.values().cloned().collect();
        values.sort_by(|a,b| b.count.cmp(&a.count));
        
        corrected.push(values[0].ch);
        corrected_after_mod.push(values.last().unwrap().ch);
    }

    println!("{} {}",corrected,corrected_after_mod);
}

pub fn run(){
    let messages = read_repetition_messages("inputs/day-6.txt");
    let arranged =arrange(&messages);
    error_correction(arranged);
}