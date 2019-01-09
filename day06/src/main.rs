#![allow(non_snake_case)]

#[macro_use]
extern crate text_io;

use std::fs;

type Coord = (i32, i32);
type Coords = Vec<Coord>;

#[derive(Copy, Clone)]
struct GridCell
{
    dist: i32,
    id: i32,
}

fn readCoords(lines: std::str::Lines) -> (Coords, i32, i32)
{
    let coords: Coords = lines.map(|line| {
        let x: i32;
        let y: i32;
        scan!(line.bytes() => "{},{}", x, y);
        (x, y)
    }).collect();

    let width  = *coords.iter().map(|(x,_)| x).max().unwrap();
    let height = *coords.iter().map(|(_,y)| y).max().unwrap();

    (coords, width, height)
}

fn manhattanDistance(coord: &Coord, x: i32, y: i32) -> i32
{
    (coord.0 - x).abs() + (coord.1 - y).abs()
}

fn processGrid(coords: &Coords, width: i32, height: i32)
{
    let mut grid = vec![GridCell{ dist: std::i32::MAX, id: coords.len() as i32 }; (width * height) as usize];

    for (id, coord) in coords.iter().enumerate() {
        for y in 0..height {
            for x in 0..width {
                let dist = manhattanDistance(coord, x, y);
                let cell = &mut grid[(y * width + x) as usize];
                if dist < cell.dist {
                    cell.dist = dist;
                    cell.id = id as i32;
                }
                else if dist == cell.dist {
                    cell.id = -1;
                }
            };
        };
    }

    let mut cellCount = vec![0i32; coords.len()];
    for y in 0..height {
        for x in 0..width {
            let cell = grid[(y * width + x) as usize];
            if cell.id >= 0 {
                cellCount[cell.id as usize] += 1;
            }
        }
    }

    for y in 0..height {
        let cell = &grid[(y * width) as usize];
        if cell.id >= 0 { cellCount[cell.id as usize] = 0; };
        let cell = &grid[(y * width + width - 1) as usize];
        if cell.id >= 0 { cellCount[cell.id as usize] = 0; };
    }

    for x in 0..width {
        let cell = &grid[x as usize];
        if cell.id >= 0 { cellCount[cell.id as usize] = 0; };
        let cell = &grid[((height - 1) * width + x) as usize];
        if cell.id >= 0 { cellCount[cell.id as usize] = 0; };
    }
    println!("size of the largest area: {}", cellCount.iter().max().unwrap());
}

fn findArea(coords: &Coords, width: i32, height: i32, max_dist: i32)
{
    let mut area = 0;
    for y in 0..height {
        for x in 0..width {
            let dist = coords.iter().map(|coord| manhattanDistance(coord, x ,y)).sum::<i32>();
            area += if dist < max_dist { 1 } else { 0 };
        }
    }
    println!("size of region: {}", area);
}

fn main() {
    let filename = "input.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let (coords, width, height) = readCoords(content.lines());
    processGrid(&coords, width, height);
    findArea(&coords, width, height, 10_000);
}
