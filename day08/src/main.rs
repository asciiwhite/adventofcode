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

fn nextEntry(entries: &Vec<usize>, id: &mut usize) -> usize {
    let res = entries[*id];
    *id += 1;
    res
}

fn valueOfNode(entries: &Vec<usize>, entryId: &mut usize) -> usize {
    let childCount = nextEntry(entries, entryId);
    let metaDataCount = nextEntry(entries, entryId);
    let childValues: Vec<_> = (0..childCount).map(|_| valueOfNode(entries, entryId)).collect();
    let metaDataSum = if childValues.is_empty() {
        (0..metaDataCount).map(|_| nextEntry(entries, entryId)).sum::<usize>()
    }
    else {
        (0..metaDataCount).filter_map(|_| childValues.get(nextEntry(entries, entryId) - 1)).sum::<usize>()
    };
    metaDataSum
}

fn parseTree(content: &str) {
    let entries: Vec<_> = content.split_whitespace().map(|w| w.parse::<usize>().unwrap()).collect();

    let mut iter = entries.iter();
    let metaDataSum = parseNode(&mut iter);
    println!("Sum of metadata: {} ", metaDataSum);

    let mut rootId = 0;
    let valueOfRootNode = valueOfNode(&entries, &mut rootId);
    println!("Value of rootNode: {} ", valueOfRootNode);
}

fn main() {
    let filename = "input.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    parseTree(&content);
}
