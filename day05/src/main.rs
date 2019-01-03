#![allow(non_snake_case)]

use std::fs;
use std::cmp;

fn reactPair(first: char, second: char) -> bool
{
    first.eq_ignore_ascii_case(&second) && first != second
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
        getNextPair(reactPair(polymer[first] as char, polymer[second] as char), &mut first, &mut second, &mut candidates, lastIndex);
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

    let minLength = (b'a'..b'z').map(|unused_char|
    {
        let charCheck = |c: u8| match !c.eq_ignore_ascii_case(&unused_char) { true => Some(c), false => None };
        let shortPolymer = reducedPolymer.iter().cloned().filter_map(|c| charCheck(initialChars[c])).collect::<Vec<u8>>();
        let furtherReducedPolymer = reducePolymer(shortPolymer.as_slice());
        println!("polymer length without '{},{}' from {} to {}", unused_char as char, unused_char.to_ascii_uppercase() as char, shortPolymer.len(), furtherReducedPolymer.len());
        furtherReducedPolymer.len()
    }).min().unwrap();

    println!("shortest polymer length: {}", minLength);
}
