#![allow(non_snake_case)]

fn main() {
    let numPlayers = 416;
    let mut playerPoints = vec![0;  numPlayers];
    let numMarbles = 71617;
    let mut currentMarblePos = 0;
    let mut currentPlayer = 0;
    let mut circle: Vec<usize> = Vec::new();
    circle.push(0);
    (1..numMarbles).for_each(|currentMarble| {
        if currentMarble % 23 == 0 {
            currentMarblePos = if currentMarblePos as i32 - 7 < 0 { circle.len() + (currentMarblePos - 7) } else { currentMarblePos - 7 };
            playerPoints[currentPlayer] += currentMarble + circle[currentMarblePos];
            circle.remove(currentMarblePos);
        }
        else
        {
            currentMarblePos += 2;
            if currentMarblePos > circle.len() { currentMarblePos -= circle.len(); } else {};
            circle.insert(currentMarblePos, currentMarble);
        }
        currentPlayer = (currentPlayer + 1) % numPlayers;
    });

//    println!("player/high score: {:?}", playerPoints);//.iter().enumerate().max_by_key(|(_, &points)| points));
    println!("player/high score: {:?}", playerPoints.iter().enumerate().max_by_key(|(_, &points)| points));
}
