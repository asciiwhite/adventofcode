#![allow(non_snake_case)]

use std::fs;
use std::cmp;

fn compareIds(lineA: &str, lineB: &str) -> (u32, usize)
{
    let iter = lineB.char_indices().zip(lineA.chars()).filter(|zip| (zip.0).1 != zip.1);
    let numNotEqualChars = iter.clone().count();
    let lastNotEqualCharId = (iter.last().unwrap().0).0;

//    println!("cmp {} and {}: {} " , lineA, lineB, numNotEqualChars);
    (numNotEqualChars as u32, lastNotEqualCharId)
}

fn calc_checksum(line: &str) -> (u32, u32)
{
    let mut char_count: [u8; 26] = [0; 26];
    line.chars().for_each(|sign| { 
        let id = sign as usize - 'a' as usize;
        char_count[id] = char_count[id].wrapping_add(1);
    });
    
    let double = char_count.iter().filter(|count| **count == 2u8).count() as u32;
    let triple = char_count.iter().filter(|count| **count == 3u8).count() as u32;
    (double, triple)
}

fn main()
{
    let filename = "././input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    
    let mut double = 0;
    let mut triple = 0;
    contents.lines().for_each(|line| {
        let (d, t) = calc_checksum(line);
        double += cmp::min(d, 1);
        triple += cmp::min(t, 1);
    });
    println!("checksum: {}", double * triple);
    
    let mut lineA = contents.lines().peekable();
    while lineA.peek() != None {
        let mut lineB = lineA.clone();
        lineB.next();
        while lineB.peek() != None {
            let (numNEChars, lastNECharId) = compareIds(lineA.peek().unwrap(), lineB.peek().unwrap());
            if numNEChars == 1 {            
                println!("found match: {} and {} at pos {}" , lineA.peek().unwrap(), lineB.peek().unwrap(), lastNECharId);
            }
            lineB.next();
        }
        lineA.next();
    }
}