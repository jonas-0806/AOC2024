use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines(r"C:\Users\Jonas\AOC2024\day2\input.txt") {
        println!("{} reports are safe",
            lines.map(| l | parse_line(l.unwrap()))
                .map(| r | report_safe(r))
                .fold(0, | acc, safe | acc + safe as u32));
    }
    else {
        println!("IO Error");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_line(line : String) -> Vec<u32> {
    line.split(' ')
        .map(| s | s.parse().unwrap())
        .collect()
}

fn report_safe(report : Vec<u32>) -> bool {
    report_increasing_legally(&report) ||
    report_increasing_legally(&report.iter().rev().cloned().collect())
}

fn report_increasing_legally(report : &Vec<u32>) -> bool {
    report.iter()
        .skip(1)
        .fold(Some(report[0]), |prev, curr| {
            match prev {
                Some(n) if *curr > n && *curr <= n + 3 => Some(*curr),
                _ => None
            }
        })
        .is_some()
}
