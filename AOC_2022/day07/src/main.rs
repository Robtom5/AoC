use std::cmp::min;
use std::collections::HashSet;
use std::fs;

fn relevant_sz((name, sz): &(String, u64), start: &str) -> u64 {
    match name.starts_with(start) {
        true => return *sz,
        false => return 0,
    }
}

fn find_values(contents: &str) -> (String, String) {
    let mut path: String = "".to_owned();
    let mut files: Vec<(String, u64)> = Vec::new();
    let mut dirs: HashSet<String> = HashSet::new();

    for line in contents.lines() {
        let words: Vec<&str> = line.split(" ").collect();
        match words[0] {
            "$" => match words[1] {
                "cd" => match words[2] {
                    ".." => {
                        let (new_path, _) = path.rsplit_once('/').unwrap();
                        path = new_path.to_string();
                    }
                    "/" => {
                        path = "".to_string();
                        dirs.insert(path.clone());
                    }
                    w => {
                        let addition = format!("{root}/{dir}", root = path, dir = w);
                        dirs.insert(addition.clone());
                        path = addition;
                    }
                },
                _ => continue,
            },
            "dir" => continue,
            sz => {
                let name = format!("{path}/{file}", path = path, file = words[1]);
                files.push((name, sz.parse::<u64>().unwrap()));
            }
        }
    }

    const MAX_SZ: u64 = 100000;
    const TOTAL_DISK_SZ: u64 = 70000000;
    const MIN_SZ: u64 = 30000000;

    let already_available: u64 =
        TOTAL_DISK_SZ - files.iter().map(|x| relevant_sz(x, "/")).sum::<u64>();
    let size_to_delete = MIN_SZ - already_available;

    assert!(MAX_SZ < size_to_delete); // We haven't accounted for this in the match statement so ensure if is is true
    let mut running_tot = 0;
    let mut best_min = u64::MAX;

    for d in dirs {
        let tot_siz: u64 = files.iter().map(|x| relevant_sz(x, &d)).sum();

        match tot_siz {
            n if n < MAX_SZ => running_tot += n,
            n if n > size_to_delete => best_min = min(best_min, n),
            _ => {}
        }
    }

    return (running_tot.to_string(), best_min.to_string());
}

fn main() {
    let fp = if cfg!(debug_assertions) {
        "data/example"
    } else {
        "data/input"
    }
    .to_string();

    let contents = fs::read_to_string(fp).expect("Should be able to read file");

    let (result1, result2) = find_values(&contents);

    println!("Part 1 {result1} Part 2 {result2}")
}
