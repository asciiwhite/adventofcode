#![allow(non_snake_case)]

use linked_list::{Cursor, LinkedList};
use std::time::{Instant};

fn go_forward<T>(cursor: &mut Cursor<T>) {
    (0..2).for_each(|_| {
        while cursor.next().is_none() {
             cursor.reset();
        };
    });
}

fn go_backwards(cursor: &mut Cursor<usize>) -> usize {
    let mut value = 0;
    (0..7).for_each(|_| {
        let mut res = cursor.prev();
        if res.is_none() { cursor.reset(); cursor.prev(); res = cursor.prev(); }
        value = *res.unwrap();
    });
    value
}

fn main() {
    let numPlayers = 416;
    let mut playerPoints = vec![0;  numPlayers];
    let numMarbles = 7161700;
    let mut currentPlayer = 0;
    let mut circle: LinkedList<usize> = LinkedList::new();
    circle.push_back(0);
    let mut cursor = circle.cursor();
    cursor.next();

    let now = Instant::now();

    (1..=numMarbles).for_each(|currentMarble| {
        if currentMarble % 23 == 0 {
            let removedMarble = go_backwards(&mut cursor);
            playerPoints[currentPlayer] += currentMarble + removedMarble;
            cursor.remove();
        }
        else {
            go_forward(&mut cursor);
            cursor.insert(currentMarble);
       }
        currentPlayer = (currentPlayer + 1) % numPlayers;
    });

    let now = now.elapsed();
    println!("{}.{}s", now.as_secs(), now.subsec_millis());
//    println!("player/high score: {:?}", playerPoints);//.iter().enumerate().max_by_key(|(_, &points)| points));
    println!("player/high score: {:?}", playerPoints.iter().enumerate().max_by_key(|(_, &points)| points));
}
