#![allow(non_snake_case)]
#![allow(dead_code)]

#[macro_use]
extern crate text_io;

use std::fs;
use std::thread;

const GRIDSIZE_WIDTH: usize = 360;
const GRIDSIZE_HEIGHT: usize = 360;

type Coord = (i32, i32);
type Coords = Vec<Coord>;

#[derive(Copy, Clone)]
struct GridCell
{
    dist: i32,
    id: i32,
}

type Grid = [[GridCell; GRIDSIZE_WIDTH]; GRIDSIZE_HEIGHT];

fn readCoords(lines: std::str::Lines) -> Coords
{
    lines.map(|line| {
        let x: i32;
        let y: i32;
        scan!(line.bytes() => "{},{}", x, y);
        (x, y)
    }).collect()
}

fn manhattanDistance(coord: &Coord, x: usize, y: usize) -> i32
{
    (coord.0 - x as i32).abs() + (coord.1 - y as i32).abs()
}

fn processGrid(grid: &mut Grid, coords: &Coords)
{
    for (id, coord) in coords.iter().enumerate() {
        for (y, row) in grid.iter_mut().enumerate() {
            for (x, col) in row.iter_mut().enumerate() {
                let dist = manhattanDistance(coord, x, y);
                if dist < col.dist {
                    col.dist = dist;
                    col.id = id as i32;
                }
                else if dist == col.dist {
                    col.id = -1;
                }
            };
        };
    }

    let mut cellCount = vec![0i32; coords.len()];
    for row in grid.iter() {
        for cell in row.iter() {
            if cell.id >= 0 {
                cellCount[cell.id as usize] += 1;
            }
        }
    }

    for row in grid.first() {
        for cell in row.iter() {
            if cell.id >= 0 { cellCount[cell.id as usize] = 0; };
         }
    }
    for row in grid.last() {
        for cell in row.iter() {
             if cell.id >= 0 { cellCount[cell.id as usize] = 0; };
         }
    }
    for row in grid.iter() {
        for cell in row.first() {
             if cell.id >= 0 { cellCount[cell.id as usize] = 0; };
         }
    }
    for row in grid.iter() {
        for cell in row.last() {
             if cell.id >= 0 { cellCount[cell.id as usize] = 0; };
         }
    }
    println!("size of the largest area: {}", cellCount.iter().max().unwrap());
}

fn findRegion(coords: &Coords, max_dist: i32)
{
    let mut area = 0;
    for y in 0..GRIDSIZE_HEIGHT {
        for x in 0..GRIDSIZE_WIDTH {
            let dist = coords.iter().map(|coord| manhattanDistance(coord, x ,y)).sum::<i32>();
            area += if dist < max_dist { 1 } else { 0 };
        }
    }
    println!("size of region: {}", area);
}

fn printGrid(grid: &Grid) {
    for row in grid.iter() {
        for col in row.iter() {
            print!("{:>2} ", col.id);
        }
        println!("");
    }
}

fn main() {
    let filename = "input.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let coords = readCoords(content.lines());

    // stack size is to small so use an separate thread with bigger stack
    let builder = thread::Builder::new().stack_size(2 * 1024 * 1024);
    let handler = builder.spawn(move || {
        let mut grid: Grid = [[GridCell{ dist: std::i32::MAX, id: coords.len() as i32 }; GRIDSIZE_WIDTH]; GRIDSIZE_HEIGHT];
        processGrid(&mut grid, &coords);
        findRegion(&coords, 10000);
//        printGrid(&grid);
    }).unwrap();

    handler.join().unwrap();
}
