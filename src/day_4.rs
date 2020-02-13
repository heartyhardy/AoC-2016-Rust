use std::env;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

#[derive(Copy, Clone,Debug, Eq, PartialEq)]
struct Charcode{
    ch:char,
    count:u8,
}

fn read_decode_instructions(filename: &str) -> String {
    let fpath = Path::new(filename);
    let abspath = env::current_dir().unwrap().into_boxed_path().join(fpath);
    let instructions = fs::read_to_string(abspath).expect("Error reading file!");
    instructions
}

fn decode_and_validate(instructions:&str){
    let mut sumof = 0;
    let mut northpole = 0;
    for line in instructions.lines(){
        let (sector, is_valid) = get_valid_checksum_sector(line.to_string());
        sumof+=sector;
        if is_valid{
            let decrypted = decrypt(line, sector);
            if decrypted.contains("northpole"){
                northpole = sector;
            }
        }
    }
    println!("{} {}", sumof, northpole);
}

fn get_valid_checksum_sector(line:String) -> (i32,bool){
    let mut char_counts:HashMap<char,Charcode> = HashMap::new();
    let last_dash:usize = line.rfind('-').unwrap();
    let code_only = line[..last_dash].trim_end();
    let sector = &line[last_dash+1..line.rfind('[').unwrap()];
    let checksum = &line[line.find('[').unwrap()+1..line.rfind(']').unwrap()];

    for ch in code_only.chars(){
        if ch.is_alphabetic(){
            if char_counts.contains_key(&ch){
                char_counts.get_mut(&ch).unwrap().count+=1;
            }else{
                let charcode =Charcode{ch,count:1};
                char_counts.insert(ch, charcode);
            }
        }
    }

    let sector:(i32,bool) = match is_real_room(char_counts, checksum){
        true => (sector.parse().unwrap(), true),
        false => (0,false),
    };
    sector    
}

fn is_real_room(char_counts:HashMap<char,Charcode>,checksum:&str)->bool{
    let mut cmp_checksum = String::new();
    let mut occurrences:Vec<Charcode> =Vec::new();

    for key in char_counts.values(){
        occurrences.push(key.clone());
    }
    occurrences.sort_by(|a,b| {
        if a.count > b.count{
            return std::cmp::Ordering::Less
        }else if a.count < b.count{
            return std::cmp::Ordering::Greater
        }
        a.ch.cmp(&b.ch)
    });
    
    for i in 0..5{
        cmp_checksum.push(occurrences[i].ch);
    }

    checksum == cmp_checksum
}

fn decrypt(line:&str,sector:i32)->String{
    let mut decrypted = String::new();
    for c in line.chars(){
        if c.is_alphabetic(){
            let ch = shift_chars(c, sector);
            decrypted.push(ch);
        }else if c == '-'{
            decrypted.push(' ');
        }
    }
    decrypted
}

fn shift_chars(code_only:char, sector:i32)-> char{
    let bcode = code_only.to_string().as_bytes()[0] + (sector % 26) as u8;
    let res:[u8;1];
    if bcode > 122{
        return (bcode-26) as char;
    }else if bcode < 97{
        return (bcode+26)as char;
    }
    return bcode as char;
}

pub fn run() {
    let instructions= read_decode_instructions("inputs/day-4.txt");
    decode_and_validate(&instructions);
}
