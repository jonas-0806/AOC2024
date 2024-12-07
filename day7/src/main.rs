fn main() {
    if let Ok(lines) = read_lines(r"C:\Users\Jonas\AOC2024\day7\input.txt") {
        println!("Result: {}", 
            lines.map(| line | parse_line(line.unwrap()))
                .fold(0, | sum, eq | sum + check_equation(eq))    
        )
    }
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Equation {
    target : u64,
    numbers : Vec<u64>
}

use std::fmt;
impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Target: {}\nNumbers: {:?}", self.target, self.numbers)
    }
}

fn parse_line(line : String) -> Equation {
    let mut split = line.split(": ");
    Equation {
        target : split.next().unwrap().parse::<u64>().unwrap(),
        numbers : split.next().unwrap().trim().split(' ').map(| n | n.parse::<u64>().unwrap()).collect()
    }
}

use std::cmp::{max, Ordering};

fn check_equation(equation : Equation) -> u64 {
    if equation.numbers.len() == 0 {
        0
    }
    else {
        check_equation_rec(&equation, 0, 0)
    }
}

fn check_equation_rec(equation : &Equation, current_result : u64, i : usize) -> u64 {
    if i == equation.numbers.len() && current_result == equation.target {
        equation.target
    }
    else {
        match current_result.cmp(&equation.target) {
        Ordering::Greater => 0,
        _ => 
            if i < equation.numbers.len() {
                max(max(
                    check_equation_rec(equation, current_result + equation.numbers.get(i).unwrap(), i + 1),
                    check_equation_rec(equation, current_result * equation.numbers.get(i).unwrap(), i + 1)),
                    check_equation_rec(equation, concatenate(current_result, equation.numbers.get(i).unwrap()), i + 1)
                )
            }
            else {
                0
            }
        }
    }
    
}

fn concatenate (a : u64, b : &u64) -> u64 {
    format!("{}{}", a, b).parse().unwrap()
}

 