#![allow(non_snake_case)]

use std::fs;
use std::cmp;
use unic_char_range::CharRange;

const CASE_DIFF: i32 = 32; //('A' as i32 - 'a' as i32).abs();

fn reactPair(first: u8, second: u8) -> bool
{
    (first as i32 - second as i32).abs() == CASE_DIFF
}

fn getNextPair(reaction: bool, first: &mut usize, second: &mut usize, candidates: &mut Vec<usize>, lastIndex: usize)
{
    match reaction {
        true => {
            if *first == 0 {
                *first = cmp::min(*second + 1, lastIndex);
                *second += 2;
            }
            else {
                *first = match candidates.pop() {
                    None => *second,
                    Some(cand) => cand,
                };
                *second = match *second == lastIndex {
                    true => cmp::max(*first, 0),
                    false => *second + 1,
                };
            } },
        false => {
            candidates.push(*first);
            *first = *second;
            *second = *first + 1;
            }
    };
}

fn reducePolymer(polymer: &[u8]) -> Vec<usize>
{
    let lastIndex = polymer.len() - 1;
    let mut first = 0;
    let mut second = 1;
    let mut candidates: Vec<usize> = Vec::new();
    
    while second <= lastIndex {
        getNextPair(reactPair(polymer[first], polymer[second]), &mut first, &mut second, &mut candidates, lastIndex);
    };
    candidates.push(second-1);
    candidates
}

fn main()
{
    let filename = "input.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
 
    let initialChars = content.as_bytes();
    let reducedPolymer = reducePolymer(initialChars);    
    println!("initial polymer length from {} to {}", initialChars.len(), reducedPolymer.len());

    let mut minLength = reducedPolymer.len();
    for char in CharRange::closed('a', 'z') {
        let lower = char as u8;
        let upper = lower - CASE_DIFF as u8;
        let shortPolymer = reducedPolymer.iter().cloned().filter_map(|c| if initialChars[c] != lower && initialChars[c] != upper { Some(initialChars[c]) } else { None }).collect::<Vec<u8>>();
        let furtherReducedPolymer = reducePolymer(shortPolymer.as_slice());
        println!("polymer length without '{},{}' from {} to {}", lower as char, upper as char, shortPolymer.len(), furtherReducedPolymer.len());
        minLength = cmp::min(furtherReducedPolymer.len(), minLength);
    }
   println!("shortest polymer length: {}", minLength);
}