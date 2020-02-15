use std::env;
use std::fmt;
use std::fs;
use std::path::Path;
use colored::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Pixel {
    Off = 0,
    On = 1,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TinyLCD {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
}

impl TinyLCD {


    // Create New LCD
    pub fn new() -> TinyLCD {
        let width = 50;
        let height = 6;

        let pixels = (0..width * height).map(|_i| Pixel::Off).collect();

        TinyLCD {
            width,
            height,
            pixels,
        }
    }


    // Read instructions file
    pub fn read_instructions(&self, filename: &str) -> String {
        let fpath = Path::new(filename);
        let abspath = env::current_dir().unwrap().into_boxed_path().join(fpath);
        let instructions = fs::read_to_string(abspath).expect("Error reading instructions!");
        instructions
    }


    // Process each instruction
    pub fn on_instruction_recieved(&mut self, instructions: &str) {
        for instruction in instructions.lines() {
            let params: Vec<&str> = instruction.split_whitespace().collect();

            match params[0] {
                "rect" => {
                    let ops: Vec<u32> = params[1].split("x").map(|i| i.parse().unwrap()).collect();
                    for r in 0..ops[1] {
                        for c in 0..ops[0] {
                            let idx = self.get_index(r, c);
                            self.pixels[idx] = Pixel::On;
                        }
                    }
                }
                "rotate" => {
                    match params[1]{
                        "row" => {
                            let r:u32 = params[2][2..].parse().unwrap();
                            let offset:u32 = params[4].parse().unwrap();
                            let row = self.copy_row(r);

                            self.turn_off_row(r);

                            for c in 0..self.width{
                                let idx = self.get_index(r, c+offset) % self.width as usize;
                                self.pixels[(r*self.width) as usize+idx] = row[c as usize];
                            }
                        },
                        "column" => {
                            let c:u32 =params[2][2..].parse().unwrap();
                            let offset:u32 = params[4].parse().unwrap();
                            let column = self.copy_column(c);

                            self.turn_off_column(c);

                            for r in 0..self.height{
                                let idx = self.get_index(r+offset, c) % (self.width * self.height) as usize;
                                self.pixels[idx] = column[r as usize];
                            }
                        },
                        _ => ()
                    }
                }
                _ => (),
            }            
        }
    }


    //Get Index by given row and column
    pub fn get_index(&self, row: u32, col: u32) -> usize {        
        (row * self.width + col) as usize
    }


    // Copy the entire row
    fn copy_row(&self, r:u32)->Vec<Pixel>{
        let mut row:Vec<Pixel> =Vec::new();
        for c in 0..self.width{
            let idx = self.get_index(r, c);
            row.push(self.pixels[idx]);
        }
        row
    }


    // Copy entire column
    fn copy_column(&self, col:u32)->Vec<Pixel>{
        let mut column:Vec<Pixel> =Vec::new();
        for r in 0..self.height{
            let idx = self.get_index(r, col);
            column.push(self.pixels[idx]);
        }
        column
    }


    // Resets all pixels in row to default
    fn turn_off_row(&mut self, r:u32){
        for c in 0..self.width{
            let idx = self.get_index(r, c);
            self.pixels[idx] = Pixel::Off;
        }
    }


    // Resets all pixels in column to default
    fn turn_off_column(&mut self, c:u32){
        for r in 0..self.height{
            let idx = self.get_index(r, c);
            self.pixels[idx] = Pixel::Off;
        }
    }

    
    // Get Pixel On/Off count
    pub fn get_count(&self,state: Pixel) ->u32{
        let mut count:u32=0;
        for p in 0..self.width*self.height{
            if self.pixels[p as usize] == state{
                count+=1;
            }
        }
        count
    }
}

impl fmt::Display for TinyLCD {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.pixels.as_slice().chunks(self.width as usize) {
            for &pixel in line {
                let symbol = if pixel == Pixel::Off { '◻' } else { '◼' };
                let symbol = match symbol{
                    '◻'=> '◻'.to_string().purple(),
                    '◼' => '◼'.to_string().bright_green(),
                    _=>' '.to_string().normal()
                };
                write!(f, "{0:>2}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub fn run() {
    let mut tinylcd = TinyLCD::new();
    let instructions = tinylcd.read_instructions("inputs/day-8.txt");
    tinylcd.on_instruction_recieved(&instructions);
    
    println!("\nPart I: ON count = {}\n\n", tinylcd.get_count(Pixel::On).to_string().green());
    println!("{0:>20}", tinylcd);
}
