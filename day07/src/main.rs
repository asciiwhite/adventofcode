#![allow(non_snake_case)]

#[macro_use]
extern crate text_io;

use std::fs;
use std::collections::HashMap;

const TIME_PER_STEP : usize = 60;
const NUM_WORKERS: usize = 5;

fn time_for_step(a: char) -> usize {
    a as usize - ('A' as usize) + 1 + TIME_PER_STEP
}

fn instructionOrder(content: &str) {
    let mut edges: HashMap<char, Vec<char>> = HashMap::new();
    let mut counts: HashMap<char, usize> = HashMap::new();

    content.lines().for_each(|line| {
        let a: char;
        let b: char;
        scan!(line.bytes() => "Step {} must be finished before step {} can begin.", a, b);
        edges.entry(a).or_default().push(b);
        edges.entry(b).or_default();
        *counts.entry(b).or_insert(0) += 1;
        counts.entry(a).or_insert(0);
    });

    while let Some(&n) = counts.iter().filter(|e| *e.1 == 0).map(|e| e.0).min() {
        print!("{}", n);
        edges[&n].iter().for_each(|e| {
            *counts.get_mut(e).unwrap() -= 1;
        });
        counts.remove(&n);
    };
}

fn stepCountWithMultipleWorkers(content: &str) {
    struct Step {
        active: bool,
        count: usize,
        time: usize
    };

    let mut edges: HashMap<char, Vec<char>> = HashMap::new();
    let mut counts: HashMap<char, Step> = HashMap::new();

    content.lines().for_each(|line| {
        let a: char;
        let b: char;
        scan!(line.bytes() => "Step {} must be finished before step {} can begin.", a, b);
        edges.entry(a).or_default().push(b);
        edges.entry(b).or_default();
        counts.entry(b).or_insert(Step{active: false, count: 0, time: time_for_step(b)}).count += 1;
        counts.entry(a).or_insert(Step{active: false, count: 0, time: time_for_step(a)});
    });

    let mut steps = 0;
    while !counts.is_empty() {
        let mut candidates = counts.iter().filter(|c| c.1.count == 0).map(|(&c,s)| (c,s.active)).collect::<Vec<_>>();
        candidates.sort_by_key(|(c,active)| *c as i32 + ((!active) as i32 * 26)); // prefer already active steps
        for (c,_) in candidates.iter().take(NUM_WORKERS) {
            let step = counts.get_mut(&c).unwrap();
            step.time -= 1;
            step.active = true;
            if step.time == 0 {
                print!("{}", c);
                for e in &edges[&c] {
                    counts.get_mut(e).unwrap().count -= 1;
                };
                counts.remove(&c);
            }
        }
        steps += 1;
    };

    println!("\nNum steps: {}", steps);
}

fn main() {
    let filename = "input.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    instructionOrder(&content);
    stepCountWithMultipleWorkers(&content);
}
