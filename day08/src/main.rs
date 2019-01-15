#![allow(non_snake_case)]

use std::fs;

fn nextIter(iter: &mut std::slice::Iter<usize>) -> usize {
    *iter.next().unwrap()
}

fn parseNode(iter: &mut std::slice::Iter<usize>) -> usize {
    let childCount = nextIter(iter);
    let metaDataCount = nextIter(iter);
    let mut metaDataSum = (0..childCount).map(|_| parseNode(iter) ).sum::<usize>();
    metaDataSum += (0..metaDataCount).map(|_| nextIter(iter) ).sum::<usize>();
    metaDataSum
}

fn valueOfNode(entries: &mut &[usize]) -> usize {
    let childCount = entries[0];
    let metaDataCount = entries[1];
    *entries = &entries[2..];
    let childValues: Vec<_> = (0..childCount).map(|_| valueOfNode(entries)).collect();
    let metaDataSum = if childValues.is_empty() {
        entries[0..metaDataCount].iter().sum::<usize>()
    }
    else {
        entries[0..metaDataCount].iter().filter_map(|c| childValues.get(c - 1)).sum::<usize>()
    };
    *entries = &entries[metaDataCount..];
    metaDataSum
}

fn parseTree(content: &str) {
    let entries: Vec<_> = content.split_whitespace().map(|w| w.parse::<usize>().unwrap()).collect();

    let mut iter = entries.iter();
    let metaDataSum = parseNode(&mut iter);
    println!("Sum of metadata: {} ", metaDataSum);

    let valueOfRootNode = valueOfNode(&mut &entries[..]);
    println!("Value of rootNode: {} ", valueOfRootNode);
}

fn main() {
    let filename = "input.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    parseTree(&content);
}
