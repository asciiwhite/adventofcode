#![allow(non_snake_case)]

use std::fs;
use std::cmp;

const CASE_DIFF: i32 = 32; //('A' as i32 - 'a' as i32).abs();

fn react(first: u8, second: u8) -> bool
{
    (first as i32 - second as i32).abs() == CASE_DIFF
}

fn getNextPairAfterReaction(first: &mut usize, second: &mut usize, candidates: &mut Vec<usize>, lastIndex: usize)
{
    if *first == 0 {
        *first = cmp::min(*second + 1, lastIndex);
        *second += 2;
    }
    else {
        if candidates.is_empty() {
            *first = *second;
//          println!("no candidate found {}", *first);
        }
        else {
            *first = candidates.pop().unwrap();
//          println!("found candidate {}", *first);
        }
        if *second == lastIndex {
            *second = cmp::max(*first, 0);
        }
        else {
            *second += 1;
        }
    }
}

fn getNextPairWithoutReaction(first: &mut usize, second: &mut usize, candidates: &mut Vec<usize>)
{
//   println!("pushed candidate {}", first);
    candidates.push(*first);
    *first = *second;
    *second = *first + 1;
}

fn main()
{
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lastIndex = contents.len() - 1;
    let mut polymerLength = contents.len();
    
    println!("initial polymer length: {}", polymerLength);
    
    let mut first = 0;
    let mut second = 1;
    let mut candidates: Vec<usize> = Vec::new();
    let chars = contents.as_bytes();
    
    while second <= lastIndex {
        assert!(first < lastIndex);
        assert!(second <= lastIndex);
        assert!(first != second);

        if react(chars[first], chars[second]) {
//            println!("found reaction: {}:{} {}:{}", first, second, chars[first] as char, chars[second] as char);
            getNextPairAfterReaction(&mut first, &mut second, &mut candidates, lastIndex);
            polymerLength -= 2;
        }
        else {
            getNextPairWithoutReaction(&mut first, &mut second, &mut candidates);
        }
//        println!("new indices: {}:{}", first, second);
    };    
    println!("final polymer length: {}", polymerLength);
}