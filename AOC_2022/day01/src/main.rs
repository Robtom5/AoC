use core::str;
use std::fs;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Elf {
    calories: i32,
}


fn elf_calories(content: &str)-> Vec<Elf>{
    let lines = content.lines();
    let mut elves: Vec<Elf> = Vec::new();

    let mut active_elf = Elf{
        calories:0
    };
    for l in lines{
        if l.is_empty(){
            elves.push(active_elf);
            active_elf = Elf{
                calories:0
            };
            continue;
        }
        active_elf.calories += match l.parse::<i32>(){
            Ok(n) => n,
            Err(_e) => panic!(),
        };
    }
    elves.push(active_elf);

    return elves
}

fn main() {
    let fp = if cfg!(debug_assertions) {
        "data/example"
    } else {
        "data/input"
    }.to_string();

    let contents = fs::read_to_string(fp)
        .expect("Should be able to read file");

    let mut elves = elf_calories(&contents);

    elves.sort_by_key(|k| -k.calories);

    #[cfg(debug_assertions)]
    for el in &elves{
        let cal = el.calories;
        println!("Elf cal {cal}")
    }

    let mut total_cal = 0;
    for i in 0..3{
        let cal = elves[i].calories;

        println!("Max Calories Elf {i}: {cal}");
        total_cal += cal;
    }
    println!("Total Calories {total_cal}");
}
