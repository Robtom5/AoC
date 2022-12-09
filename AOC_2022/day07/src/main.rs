use std::collections::HashSet;
use std::fs;

fn part1(contents: &str) -> String {
    // let sumofsmall = sum_small_contents(contents, 10000);
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

    let mut running_tot = 0;

    for d in dirs {
        fn relevant_sz((name, sz): &(String, u64), start: &str) -> u64 {
            match name.starts_with(start) {
                true => return *sz,
                false => return 0,
            }
        }
        let tot_siz: u64 = files.iter().map(|x| relevant_sz(x, &d)).sum();
        match tot_siz {
            n if n < 100000 => running_tot += n,
            _ => {}
        }
    }

    return running_tot.to_string();
}

fn main() {
    let fp = if cfg!(debug_assertions) {
        "data/example"
    } else {
        "data/input"
    }
    .to_string();

    let contents = fs::read_to_string(fp).expect("Should be able to read file");

    let result1 = part1(&contents);

    println!("Part 1 {result1}")
}
