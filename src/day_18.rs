use std::env;
use std::fs;
use std::fmt;
use std::path::Path;

const ROWS:usize = 400000;
const COLUMNS:usize = 100;

fn readTileData(filename: &str) -> String{
    let fpath = Path::new(filename);
    let abspath = env::current_dir()
        .unwrap()
        .into_boxed_path()
        .join(fpath);
    let content = fs::read_to_string(abspath)
        .expect("Error Reading File!");
    content
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Tile{
    Safe = 0,
    Trap = 1
}

#[derive(Clone)]
struct Dungeon {
    tiles: Vec<Vec<Tile>>   
}

impl Dungeon{
    pub fn new() -> Dungeon{
        let mut tiles:Vec<Vec<Tile>> = Vec::new();
        
        for row in 0..ROWS{
            tiles.push(Vec::new());
            for col in 0..COLUMNS{
                tiles[row].push(Tile::Safe);
            }
        }

        let mut dungeon = Dungeon{
            tiles: tiles,
        };
        dungeon.initialize_tiles();
        dungeon.scan_all();
        dungeon     
    }

    fn initialize_tiles(&mut self){
        let tiledata = readTileData("inputs/day-18.txt");
        for (i,ch) in tiledata.chars().enumerate(){
            match ch{
                '.' => self.tiles[0][i] = Tile::Safe,
                '^' => self.tiles[0][i] = Tile::Trap,
                _ => ()
            }
        }
    }

    fn scan_traps_in_row(&mut self, row: usize){
        if row == 0 || row >= ROWS{
            return;
        }

        for col in 0..COLUMNS{
            let mut l = Tile::Safe;
            let mut c = Tile::Safe;
            let mut r = Tile::Safe;
            let mut is_trapped = false;

            if (col as i32 -1 >= 0){
                l = self.tiles[row-1][col-1];
            }
            if (col+1 < COLUMNS){
                r = self.tiles[row-1][col+1];
            }

            c = self.tiles[row-1][col];

            if l == Tile::Trap && c == Tile::Trap && r == Tile::Safe{
                is_trapped = true;
            }else if r == Tile::Trap && c == Tile::Trap && l == Tile::Safe{
                is_trapped = true;
            }else if l == Tile::Trap && c == Tile::Safe && r == Tile::Safe{
                is_trapped = true;
            }else if r == Tile::Trap && c == Tile::Safe && l == Tile::Safe{
                is_trapped = true;
            }else{
                is_trapped = false;
            }

            if is_trapped{
                self.tiles[row][col] = Tile::Trap;
            } else{
                self.tiles[row][col] = Tile::Safe;
            }
        }
    }

    fn scan_all(&mut self){
        for row in 1..ROWS{
            self.scan_traps_in_row(row);
        }
    }

    pub fn count(&self, tile_type:Tile) -> usize{
        let (mut safe, mut trapped) = (0,0);

        for row in 0..ROWS{
            for col in 0..COLUMNS{
                match self.tiles[row][col]{
                    Tile::Safe => safe+=1,
                    Tile::Trap =>trapped+=1,
                }
            }
        }
        if tile_type == Tile::Safe{
            return safe
        }
        return trapped
    }
}

impl fmt::Display for Dungeon{
    fn fmt(&self, f: &mut fmt::Formatter) ->fmt::Result{
        for r in 0..ROWS{
            for c in 0..COLUMNS{
                let symbol = if self.tiles[r][c] == Tile::Safe { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}


pub fn run(){
    let dungeon = Dungeon::new();
    //println!("{}", dungeon);
    println!("\nSafe: {} Trapped: {}",dungeon.count(Tile::Safe), dungeon.count(Tile::Trap) );
}