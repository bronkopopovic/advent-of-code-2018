extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

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

fn print_logs(_logs: Vec<Log>) {
    for _log in _logs {    
        match _log.action {
            LogType::GuardBegins(_id) => {
                println!("{}-{}-{}, {}:{} - Guard {} starts", _log.date.year, _log.date.month, _log.date.day, _log.date.hour, _log.date.minute, _id);
            },
            LogType::FallAsleep => {
                println!("{}-{}-{}, {}:{} - falls asleep", _log.date.year, _log.date.month, _log.date.day, _log.date.hour, _log.date.minute);
            },
            LogType::WakeUp => {
                println!("{}-{}-{}, {}:{} - wakes up", _log.date.year, _log.date.month, _log.date.day, _log.date.hour, _log.date.minute);
            }
            _ => {
                println!("none");
            }
        }
    }
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

fn main() {

    let mut content = String::new();

    File::open("src/input.txt").expect("dafuq?")
        .read_to_string(&mut content).expect("something went wrong reading the file");

    let mut logs: Vec<Log> = Vec::new();

    populate_logs(content, &mut logs);

    sort_logs(&mut logs);

    print_logs(logs);
    
}
