#![allow(non_snake_case)]

use std::fs;
use regex::Regex;

static mut MAP: [[usize; 1000]; 1000] = [[0; 1000]; 1000];
static mut CLAIMS: [bool; 1287] = [false; 1287];

fn insertInMap(id: usize, x: u32, y: u32, w: u32, h: u32) -> u32
{
    let mut squaresInMoreThanOneClaim = 0;
    unsafe {
        let mut square_count = 0;
        for _y in y..y+h {
            for _x in x..x+w {
                let m = &mut MAP[_y as usize][_x as usize];
                match m {
                    0 => { square_count += 1; },
                    _ => { squaresInMoreThanOneClaim += CLAIMS[*m-1] as u32; CLAIMS[*m-1] = false; },
                }
                *m = id;
            }
        }
        if square_count == w*h {
            CLAIMS[id-1] = true;
        }
    }
    squaresInMoreThanOneClaim
}

fn main() {
    let filename = "././input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    
    let re = Regex::new(r"#(\d{1,}) @ (\d{1,}),(\d{1,}): (\d{1,})x(\d{1,})").unwrap();
    let mut squaresInMoreThanOneClaim = 0;
    for line in contents.lines() {
        let cap = re.captures_iter(line).next().unwrap();
//        println!("id:{} pos: {}x{} size: {}x{}", &cap[1], &cap[2], &cap[3], &cap[4], &cap[5]);
        squaresInMoreThanOneClaim += insertInMap(
            cap[1].parse::<usize>().unwrap(),
            cap[2].parse::<u32>().unwrap(),
            cap[3].parse::<u32>().unwrap(),
            cap[4].parse::<u32>().unwrap(),
            cap[5].parse::<u32>().unwrap());
    }
    println!("num square inches: {}", squaresInMoreThanOneClaim);
    
    unsafe {
        println!("non overlapping clain id: {}", CLAIMS.iter().position(|&x| x == true).unwrap() + 1);
   }
}
