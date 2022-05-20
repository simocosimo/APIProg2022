use std::env;
use chrono::NaiveTime;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Calendar {
    schedule: Vec<(NaiveTime, NaiveTime)>,
    bounds: (NaiveTime, NaiveTime)
}

impl Calendar {
    pub fn new(schedule: Vec<(NaiveTime, NaiveTime)>, bounds: (String, String)) -> Self{
        Self { schedule, bounds: str_to_naive(bounds) }
    }

    pub fn from_file(file: File) -> Self{
        let buff = BufReader::new(file);
        let mut bounds= (String::new(), String::new());
        let mut schedule= Vec::new();

        for (i, line) in buff.lines().enumerate() {
            let l =  line.unwrap();
            let str_parts: Vec<&str> = l.split(" ").collect();
            let app = (str_parts[0].to_string(), str_parts[1].to_string());
            if i == 0 { bounds = app; }
            else { schedule.push(str_to_naive(app)); }
        }
        Calendar::new(schedule, bounds)
    }
}

fn str_to_naive(tuple: (String, String)) -> (NaiveTime, NaiveTime) {
    let b = (
        NaiveTime::parse_from_str(tuple.0.as_str(), "%H:%M").unwrap(),
        NaiveTime::parse_from_str(tuple.1.as_str(), "%H:%M").unwrap(),
    );
    b
}

fn find_time(cal1: Calendar, cal2: Calendar) -> Vec<(NaiveTime, NaiveTime)> {
    let min_max_bounds = (
        cal1.bounds.0.max(cal2.bounds.0),
        cal1.bounds.1.min(cal2.bounds.1),
        );
    let mut last_time = min_max_bounds.0;
    let mut res = Vec::new();
    for (c1, c2) in cal1.schedule.iter().zip(cal2.schedule.iter()) {
        let mut min = c1.0.min(c2.0);
        if min < last_time { min = c1.0.max(c2.0); }
        res.push((last_time, min));
        last_time = c1.1.max(c1.0);
    };
    res
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_cal1 = File::open(args[1].as_str());
    let input_cal2 = File::open(args[2].as_str());
    let min_duration = args[3].as_str();

    if input_cal1.is_err() || input_cal2.is_err() {
        println!("Cannot open one of the specified files.");
        println!("{}", input_cal1.unwrap_err());
        println!("{}", input_cal2.unwrap_err());
        return;
    }

    let cal1 = Calendar::from_file(input_cal1.unwrap());
    let cal2 = Calendar::from_file(input_cal2.unwrap());

    println!("Calendar 1: ");
    for s in cal1.schedule {
        println!("{:?}", s);
    }

    println!("Calendar 2: ");
    for s in cal2.schedule {
        println!("{:?}", s);
    }

    println!("Looking for time interval of {} minutes...", min_duration);
}
