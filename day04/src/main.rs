#![allow(non_snake_case)]

use std::fs;
use std::collections::HashMap;
use regex::Regex;
use chrono::NaiveDateTime;
use chrono::Timelike;

#[derive(Debug)]
enum GuardAction {
    BeginShift(u32),
    FallAsleep,
    WakeUp
}

#[derive(Debug)]
struct TimeAndAction {
    time: NaiveDateTime,
    action: GuardAction,
}

fn extractGuardAction(guardRe: &Regex, actionString: &str) -> GuardAction
{    
    let cap = guardRe.captures(actionString);
    if cap.is_none() {
        match actionString {
            "wakes up"      => GuardAction::WakeUp,
            "falls asleep"  => GuardAction::FallAsleep,
            _               => GuardAction::FallAsleep,
        }
    }
    else {
        GuardAction::BeginShift(cap.unwrap()[1].parse().unwrap())
    }
}

fn readAndSortEvents(filename: &str) -> Vec<TimeAndAction>
{
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut events: Vec<TimeAndAction> = Vec::new();

    let guardRe = Regex::new(r"Guard #(\d{1,}) (.+)").unwrap();
    let re = Regex::new(r"\[(.+)] (.+)").unwrap();
    for line in contents.lines() {
        let cap = re.captures(line).unwrap();
        let time = NaiveDateTime::parse_from_str(&cap[1], "%Y-%m-%d %H:%M").unwrap();
        let action = extractGuardAction(&guardRe, &cap[2]);
        events.push(TimeAndAction{ time, action });
    }
    events.sort_by_key(|x| x.time);
    events
}

struct GuardMinutes {
    minutes: [u32; 60]
}

impl GuardMinutes {
    fn new() -> GuardMinutes {
        GuardMinutes { minutes: [0; 60] }
    }
}

fn main()
{
    let filename = "input.txt";
    let events = readAndSortEvents(filename);
//    println!("{:#?} #events: {}", events, events.len());
    
    let mut activeGuard = 0u32;
    let mut guardMinutes: HashMap<u32, GuardMinutes> = HashMap::new();
    let mut startSleepingMinute = 0;
    for event in events {
        match event.action {
           GuardAction::BeginShift(guard) => activeGuard = guard,
           GuardAction::WakeUp => {
//                println!("guard {} sleeping from {} till {}", activeGuard, startSleepingMinute, event.time.minute());
                let entry = guardMinutes.entry(activeGuard).or_insert(GuardMinutes::new());
                for i in startSleepingMinute..event.time.minute() {
                    entry.minutes[i as usize] += 1;
                }
           },
           GuardAction::FallAsleep => startSleepingMinute = event.time.minute(),
        }
    }

    let functionalTestResult = guardMinutes.iter().map(|(guardId, minutes)| (*guardId, minutes.minutes.iter().sum::<u32>(), minutes)).max_by_key(|x| x.1).map(|x| 
        x.2.minutes.iter().position(|a| a == x.2.minutes.iter().max().unwrap()).unwrap() * x.0 as usize);
    println!("{}", functionalTestResult.unwrap());
    
    let mut maxSum = 0;
    let mut maxGuardId = 0;
    let mut maxMinute = 0;
    for (guardId, minutes) in &mut guardMinutes {
        let sum = minutes.minutes.iter().sum();
        if maxSum < sum {
            maxSum = sum;
            maxGuardId = *guardId;
            let max = minutes.minutes.iter().max().unwrap();
            maxMinute = minutes.minutes.iter().position(|x| x == max).unwrap();
        }
    }
    println!("strategy 1: guard {}: minute: {} answer: {}", maxGuardId, maxMinute, maxGuardId as usize * maxMinute);
    
    let mut maxMinute = 0;
    let mut maxGuardId = 0;
    let mut maxTime = 0u32;
    for (guardId, minutes) in guardMinutes {
        let max = minutes.minutes.iter().max().unwrap();
        if maxTime < *max {
            maxMinute = minutes.minutes.iter().position(|x| x == max).unwrap();
            maxTime = *max;
            maxGuardId = guardId;            
        }
    }
    println!("strategy 2: guard {}: minute: {} answer: {}", maxGuardId, maxMinute, maxGuardId as usize * maxMinute);
}
