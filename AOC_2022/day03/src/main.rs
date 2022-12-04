use std::fs;
use std::collections::HashSet;

fn parse_backpack_contents(content: &str) -> Vec<char> {
    let mut extra_items: Vec<char> = Vec::new();
    let lines = content.lines();
    for line in lines {
        let len = line.chars().count() / 2;
        let mut fh = HashSet::new();
        for c in line[0..len].chars(){
            fh.insert(c);
        }
        let mut sh = HashSet::new();
        for c in line[len..].chars(){
            sh.insert(c);
        }

        let diff = fh.intersection(&sh).collect::<Vec<&char>>();

        match diff.len(){
            1 => extra_items.push(*diff[0]),
            _ => panic!("")
        }      
    }
    return extra_items;
}

fn find_badge(content: &str) -> Vec<char> {
    let mut badges: Vec<char> = Vec::new();
    let lines: Vec<&str> = content.lines()
        .collect();
    for i in (0..lines.len()).step_by(3){
        let l1: HashSet<char> = HashSet::from_iter(lines[i].chars());
        let l2: HashSet<char> = HashSet::from_iter(lines[i+1].chars());
        let l3: HashSet<char> = HashSet::from_iter(lines[i+2].chars());

        let diff1 = l1.intersection(&l2);
        let mut l12 = HashSet::new();
        for d in diff1{
            l12.insert(*d);
        }
        let diff2 = l3.intersection(&l12)
            .collect::<Vec<&char>>();

        match diff2.len(){
            1 => badges.push(*diff2[0]),
            _ => panic!("")
        }      
    }
    return badges;
}

fn prioritise(errors: Vec<char>) -> u32 {
    let mut total_priorities = 0;
    let priorities:Vec<char>= "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(); 

    for c in errors{
        let priority =  priorities.iter().position(| &x| x == c).expect("Can't find {c}");
        total_priorities += priority as u32 + 1;

    }
    return total_priorities;
}

fn main() {
    let fp = if cfg!(debug_assertions) {
        "data/example"
    } else {
        "data/input"
    }.to_string();

    let contents = fs::read_to_string(fp)
        .expect("Should be able to read file");

    let wrong = parse_backpack_contents(&contents);
    let total_score = prioritise(wrong);

    let badges = find_badge(&contents);
    let badge_score = prioritise(badges);

    println!("Total score {total_score}");
    println!("Badge score {badge_score}");

    // intersection of all 3
}