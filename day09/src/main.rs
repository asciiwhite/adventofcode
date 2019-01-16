#![allow(non_snake_case)]

use linked_list::{Cursor, LinkedList};
use std::time::{Instant};

fn go_forward<T>(cursor: &mut Cursor<T>) {
    for _ in 0..2 {
        while cursor.next().is_none() {
             cursor.reset();
        };
    };
}

fn go_backwards(cursor: &mut Cursor<usize>) {
    for _ in 0..7 {
        if cursor.prev().is_none() { cursor.reset(); cursor.prev(); cursor.prev(); }
    };
}

fn main() {
    let numPlayers = 416;
    let mut playerPoints = vec![0;  numPlayers];
    let numMarbles = 7161700;
    let mut circle: LinkedList<usize> = LinkedList::new();
    circle.push_back(0);
    let mut cursor = circle.cursor();
    cursor.next();

    let now = Instant::now();

    for (marble, player) in (1..=numMarbles).zip((0..numPlayers).cycle()) {
        if marble % 23 == 0 {
            go_backwards(&mut cursor);
            playerPoints[player] += marble + cursor.remove().unwrap();
        }
        else {
            go_forward(&mut cursor);
            cursor.insert(marble);
       }
    };

    let now = now.elapsed();
    println!("{}.{}s", now.as_secs(), now.subsec_millis());
//    println!("player/high score: {:?}", playerPoints);//.iter().enumerate().max_by_key(|(_, &points)| points));
    println!("player/high score: {:?}", playerPoints.iter().enumerate().max_by_key(|(_, &points)| points));
}
