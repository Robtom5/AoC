use regex::Regex;
use std::fs;

mod solns01_05;
mod solns06_10;
mod solns11_15;

fn main() {
    for i in 1..25 {
        let fh = match cfg!(debug_assertions) {
            true => format!("data/{:02}ex", i),
            false => format!("data/{:02}in", i),
        };

        let contents = match fs::read_to_string(&fh) {
            Ok(n) => n,
            Err(_) => continue,
        };

        let (_soln1, _soln2) = match i {
            1 => solns01_05::day01(&contents),
            2 => solns01_05::day02(&contents),
            3 => solns01_05::day03(&contents),
            4 => solns01_05::day04(&contents),
            5 => solns01_05::day05(&contents),
            6 => solns06_10::day06(&contents),
            7 => solns06_10::day07(&contents),
            8 => solns06_10::day08(&contents),
            9 => solns06_10::day09(&contents),
            10 => solns06_10::day10(&contents),
            11 => solns11_15::day11(&contents),
            12 => solns11_15::day12(&contents),
            13 => solns11_15::day13(&contents),
            14 => solns11_15::day14(&contents),
            _ => continue,
        };

        println!("Day {i:02}\t Part 1: {_soln1:<16} Part 2: {_soln2:<16}");
    }
}
