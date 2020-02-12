use std::env;
use std::fs;
use std::path::Path;

fn read_triangles_list(filename:&str)->String{
    let fpath = Path::new(filename);
    let abspath = env::current_dir()
        .unwrap()
        .into_boxed_path()
        .join(fpath);
    let list = fs::read_to_string(abspath)
    .expect("Error reading file!");
    list
}

fn count_triangles_horizontally(triangles_list:&str) -> i32{
    let mut count = 0;
    for sides_list in triangles_list.lines(){
        let sides:Vec<i32> = sides_list
            .split_whitespace()
            .map(|e| e.parse().unwrap())
            .collect();

        if is_valid_triangle(sides) == true{
            count+=1;
        };
    }
    count
}

fn count_triangles_vertically(triangles_list:&str)->i32{
    let mut count = 0;
    let mut c1:Vec<i32> =Vec::new();
    let mut c2:Vec<i32>=Vec::new();
    let mut c3:Vec<i32>=Vec::new();

    for (i,sides_list) in triangles_list.lines().enumerate(){
        let sides:Vec<i32> = sides_list
            .split_whitespace()
            .map(|e| e.parse().unwrap())
            .collect();
        
        c1.push(sides[0]);
        c2.push(sides[1]);
        c3.push(sides[2]);

        let mut s1:Vec<i32> = Vec::new();
        let mut s2:Vec<i32> = Vec::new();
        let mut s3:Vec<i32> = Vec::new();

        if (i+1)%sides.len() == 0 && i != 0{
            s1.extend_from_slice(&c1[i-2..]);
            s2.extend_from_slice(&c2[i-2..]);
            s3.extend_from_slice(&c3[i-2..]);

            if is_valid_triangle(s1) == true{
                count+=1;
            };
            if is_valid_triangle(s2) == true{
                count+=1;
            };
            if is_valid_triangle(s3) == true{
                count+=1;
            };
        }
    }
    count
}

fn is_valid_triangle(sides:Vec<i32>)->bool{
    for i in 0..sides.len(){
        let l=&sides[..i];
        let r=&sides[i+1..];
        let merged = [l,r].concat();

        if sides[i] >= merged.iter().sum(){
            return false;
        }
    }
    return true
}

pub fn run(){
    let triangles_list =read_triangles_list("inputs/day-3.txt");
    let count_h = count_triangles_horizontally(&triangles_list);
    println!("Part I: {}", count_h);

    let count_v = count_triangles_vertically(&triangles_list);
    println!("Part II: {}", count_v);
}