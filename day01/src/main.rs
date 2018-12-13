#![allow(non_snake_case)]

use std::fs;
use std::collections::HashSet;

fn main()
{
    let filename = "frequences.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    
    let sum: i32 = contents.lines().map(|l| l.parse::<i32>().unwrap()).sum();
    println!("freq sum: {}", sum);

    let mut doubled = 0;
    let mut freqs = HashSet::new();
    {
        let mut checkFrequence = |f| if !freqs.insert(doubled) { true } else { doubled += f; false };
        let _ = contents.lines().cycle().position(|l| checkFrequence(l.parse::<i32>().unwrap()));
    }            
    println!("first doubled frequence: {}", doubled);
}