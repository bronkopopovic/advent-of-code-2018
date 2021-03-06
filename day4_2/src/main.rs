extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

enum LogType {
    FallAsleep,
    WakeUp,
    GuardBegins(i32),
    NoType,
}

struct Date {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
}

struct Log {
    date: Date,
    action: LogType,
}

fn populate_logs(_content: String, _logs: &mut Vec<Log>) {

    let log_regex = Regex::new(r"\[(\d{4})\-(\d{2})\-(\d{2})\s(\d{2}):(\d{2})\]\s(.+)").unwrap();
    let guard_regex = Regex::new(r"\#(\d+)").unwrap();
    
    for line in _content.split("\n") {        
    
        let caps = log_regex.captures(&line).unwrap();
        
        let date = Date {
            year: caps[1].parse::<i32>().expect("not a num"),
            month: caps[2].parse::<i32>().expect("not a num"),
            day: caps[3].parse::<i32>().expect("not a num"),
            hour: caps[4].parse::<i32>().expect("not a num"),
            minute: caps[5].parse::<i32>().expect("not a num"),
        };
        
        let action = caps[6].parse::<String>().expect("not a string");
        let log_type: LogType;

        match action.chars().next().as_ref() {
            Some(_val) => {
                match _val {
                    'f' => {
                        log_type = LogType::FallAsleep;
                    },
                    'w' => {
                        log_type = LogType::WakeUp;
                    },
                    'G' => {
                        let guard_cap = guard_regex.captures(&action).unwrap();
                        let guard_id = guard_cap[1].parse::<i32>().expect("not a num");
                        log_type = LogType::GuardBegins(guard_id);
                    },
                    _ => {
                        log_type = LogType::NoType;        
                    }
                }
            },
            None => {
                log_type = LogType::NoType;
            }
        }

        let log = Log {
            date: date,
            action: log_type,
        };

        _logs.push(log);
    }
}

fn sort_logs(_logs: &mut Vec<Log>) {
    _logs.sort_by( |a, b| {
        a.date.year.cmp(&b.date.year)
            .then( a.date.month.cmp(&b.date.month) )
            .then( a.date.day.cmp(&b.date.day) )
            .then( a.date.hour.cmp(&b.date.hour) )
            .then( a.date.minute.cmp(&b.date.minute) )
    });
}

fn populate_guards(_guards: &mut HashMap<i32, HashMap<i32, i32>>, _logs: &mut Vec<Log>) {
    let mut sleep_start = 0;
    let mut curr_guard = 0;

    for _log in _logs {

        match _log.action {
            LogType::GuardBegins(_id) => {
                curr_guard = _id;
            },
            LogType::FallAsleep => {
                sleep_start = _log.date.minute;
            },
            LogType::WakeUp => {                
                
                for _min in sleep_start .. _log.date.minute {

                    let mut _temp_hash: HashMap<i32, i32> = HashMap::new();
                    
                    match _guards.get(&curr_guard) {
                        Some(_guard) => {

                            _temp_hash = _guard.clone();
                            
                            match _temp_hash.get(&_min) {
                                Some(_occurance) => {
                                    _temp_hash.insert(_min.clone(), _occurance + 1);
                                },
                                None => {
                                    _temp_hash.insert(_min.clone(), 1);
                                }
                            }
                            
                        },
                        None => {
                            _temp_hash.insert(_min.clone(), 1);
                        }
                    }

                    _guards.insert(curr_guard.clone(), _temp_hash);
                }
            },
            _ => {}
        }
    }
}

fn get_target_guard(_guards: &mut HashMap<i32, HashMap<i32, i32>>) {
    let mut target = 0;
    let mut highest_minute = 0;
    let mut highest_occurance = 0;

    for (guard, minutes) in _guards.iter() {
        for (minute, occurance) in minutes.iter() {
            if *occurance > highest_occurance {
                target = *guard;
                highest_minute = *minute;
                highest_occurance = *occurance;
            }
        }
    }

    println!("Target Guard = {}", target);
    println!("Was asleep at minute {} {} times", highest_minute, highest_occurance);
    println!("Result = {}", target * highest_minute);   
}

fn main() {

    let mut content = String::new();

    File::open("src/input.txt").expect("dafuq?")
        .read_to_string(&mut content).expect("something went wrong reading the file");

    let mut logs: Vec<Log> = Vec::new();

    populate_logs(content, &mut logs);
    
    sort_logs(&mut logs);

    let mut guards: HashMap<i32, HashMap<i32, i32>> = HashMap::new();

    populate_guards(&mut guards, &mut logs);

    get_target_guard(&mut guards);

}
