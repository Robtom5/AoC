use std::fs;

fn part1(contents: &str) -> String {
    // let sumofsmall = sum_small_contents(contents, 10000);
    let mut path: String = "".to_owned();
    let mut files: Vec<(String, u64)> = Vec::new();

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
                    }
                    w => {
                        let addition = format!("{root}/{dir}", root = path, dir = w);
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

    for pair in files {
        let (file, _sz) = pair;
        println!("{file}")
    }
    return "sumofsmall".to_string();
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
