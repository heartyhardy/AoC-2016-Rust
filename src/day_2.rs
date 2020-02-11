use std::env;
use std::fs;
use std::path::Path;

struct Direction {
    x: i32,
    y: i32,
}

fn read_bathroom_codes(filename: &str) -> String {
    let fpath = Path::new(filename);
    let abspath = env::current_dir().unwrap().into_boxed_path().join(fpath);
    let codes = fs::read_to_string(abspath).expect("Error reading file!");
    codes
}

fn enter_key_code(codes: &str, keypad_id:u8) {
    let keypad_i: [[u8; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7,8,9]];
    let keypad_ii: [[u8; 5]; 5] = [[0, 0, 1, 0, 0], [0, 2, 3, 4, 0], [5,6,7,8,9],[0,10,11,12,0],[0,0,13,0,0]];

    let directions: [Direction; 4] = [
        Direction { x: 0, y: -1 }, //U
        Direction { x: 1, y: 0 },  //R
        Direction { x: 0, y: 1 },  //D
        Direction { x: -1, y: 0 }, //L
    ];
    let (mut x, mut y) = match keypad_id{
        1 => (1,1),
        2 => (0,2),
        _ => (0,0)
    };

    for line in codes.lines() {
        let codeset: Vec<&str> = line
        .split("")
        .filter(|x| !x.is_empty())
        .collect();
        
        for code in codeset.iter() {
            let (mut dx, mut dy) = (0, 0);
            match code {
                &"U" => {
                    dx = x + directions[0].x;
                    dy = y + directions[0].y;
                }
                &"R" => {
                    dx = x + directions[1].x;
                    dy = y + directions[1].y;
                }
                &"D" => {
                    dx = x + directions[2].x;
                    dy = y + directions[2].y;
                }
                &"L" => {
                    dx = x + directions[3].x;
                    dy = y + directions[3].y;
                }
                _ => (),
            }


            if (dx >= 0 && dx < 3) && (dy >= 0 && dy < 3) && keypad_id== 1 {
                x = dx;
                y = dy;
            }else if (dx >= 0 && dx < 5) && (dy >= 0 && dy < 5) && keypad_id == 2{
                if keypad_ii[dy as usize][dx as usize] != 0{
                    x = dx;
                    y = dy;
                }else{
                    continue;
                }
            }
        }
        if keypad_id == 1{
            print!("{}", keypad_i[y as usize][x as usize])
        }else if keypad_id == 2{
            match keypad_ii[y as usize][x as usize]{
                10 => print!("{}", "A"),
                11 => print!("{}", "B"),
                12 => print!("{}", "C"),
                13 => print!("{}", "D"),
                _=> print!("{}", keypad_ii[y as usize][x as usize])
            }
        }
    }
}

pub fn run() {
    let codes = read_bathroom_codes("inputs/day-2.txt");
    println!("PART I{}", "");
    enter_key_code(&codes,1);
    println!("\nPART II{}", "");
    enter_key_code(&codes,2);
    println!("{}", "");
}
