use std::env;
use std::fs;
use std::path::Path;

fn read_ip7_list(filename: &str) -> String {
    let fpath = Path::new(filename);
    let abspath = env::current_dir().unwrap().into_boxed_path().join(fpath);
    let list = fs::read_to_string(abspath).expect("Error reading file");
    list
}

fn is_abba(slice: &str) -> bool {
    let mut in_hypernet = false;
    let mut valid = false;
    let slice: Vec<_> = slice.chars().collect();

    for window in slice.windows(4) {
        if window[0] == '[' || window[0] == ']' {
            in_hypernet = !in_hypernet;
            continue;
        }

        if window[0] != window[1] && window[1] == window[2] && window[0] == window[3] {
            if in_hypernet {
                return false;
            } else {
                valid = true;
            }
        }
    }

    valid
}

fn is_aba(slice: &str) -> bool {
    let (mut abas, mut babs) = (Vec::new(), Vec::new());
    let mut in_hypernet = false;
    let slice: Vec<_> = slice.chars().collect();

    for window in slice.windows(3) {
        if window[0] == '[' || window[0] == ']' {
            in_hypernet = !in_hypernet;
            continue;
        }

        if window[0] == window[2] && window[0] != window[1] {
            if in_hypernet {
                babs.push((window[1], window[0], window[1]));
            } else {
                abas.push((window[0], window[1], window[0]));
            }
        }
    }

    for aba in &abas {
        for bab in &babs {
            if aba == bab {
                return true;
            }
        }
    }

    false
}

pub fn adv_main(input: Vec<String>) {
    let (tls, ssl) = input
        .iter()
        .map(|s| (is_abba(s) as u64, is_aba(s) as u64))
        .fold((0, 0), |(t, s), (tls, ssl)| (t + tls, s + ssl));

    println!("tls support: {}\nssl support: {}", tls, ssl);
}

pub fn run() {
    let ins = read_ip7_list("inputs/day-7.txt");
    let list:Vec<String> = ins.split("\n").map(|e|e.to_string()).collect();
    adv_main(list);
}
