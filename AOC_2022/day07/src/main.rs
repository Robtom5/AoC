use core::str::Lines;
use std::fs;

struct Node {
    name: String,
    // parent: Option<Box<Node>>,
    file_size: u32,
    children: Vec<Node>,
}

fn new_node(name: String) -> Node {
    Node {
        name: name,
        // parent: None,
        file_size: 0,
        children: Vec::new(),
    }
}

// trait

// fn sum_small_contents(contents: &str, max: u32) -> u32 {
//     return running_sum(contents.lines(), max);
// }

// fn running_sum(mut lines: Lines, max: u32) -> u32 {
//     let mut dir_sum = 0;
//     let line = lines.next().unwrap();
//     let (word1, _word2) = line.split_once(' ').unwrap();
//     println!("{max}");

//     dir_sum += match word1.chars().next().unwrap() {
//         '$' => 0,                       // Command
//         'd' => running_sum(lines, max), // Directory
//         _ => word1.parse::<u32>().unwrap(),
//     };
//     return match dir_sum {
//         n if n < max => n,
//         _ => 0,
//     };
// }
fn load_folder(mut contents: &str) -> usize {
    let mut folder_size = 0;
    loop {
        let next_line_break = match contents.find('\n') {
            Some(n) => n,
            None => break,
        };
        let line = &contents[..next_line_break];
        contents = &contents[next_line_break..];

        let words: Vec<&str> = line.split(' ').collect();
        let mut word_iter = words.iter();

        let word1 = word_iter.next().unwrap();

        match *word1 {
            "$" => {
                let word2 = word_iter.next().unwrap();
                match *word2 {
                    "ls" => return 2,
                    "cd" => {
                        let word3 = word_iter.next().unwrap();
                        match *word3 {
                            ".." => 3, // Got back
                            _ => load_folder(contents),
                        }
                    }
                    _ => panic!("Unknown word {word2}"),
                };
            } // Command
            "dir" => {}

            _ => {
                folder_size += word1.parse::<usize>().unwrap();
            }
        }
    }
    return folder_size;
}

fn part1(contents: &str) -> String {
    // let sumofsmall = sum_small_contents(contents, 10000);
    let mut lines = contents.lines();
    lines.next(); // We dont need the first two lines
    lines.next();
    let root: Node = new_node("/".to_string());
    let mut active_node = &root;

    for line in lines {
        let words: Vec<&str> = line.split(' ').collect();
        let mut word_iter = words.iter();

        let word1 = word_iter.next().unwrap();

        match *word1 {
            "$" => {
                let word2 = word_iter.next().unwrap();
                match *word2 {
                    "ls" => {}
                    "cd" => {
                        let word3 = word_iter.next().unwrap();
                        let &child_node = new_node(word3.to_string());

                        active_node.children.push(child_node);
                        active_node = &child_node;
                    }
                    _ => panic!("Unknown word {word2}"),
                }
            } // Command
            "dir" => {}

            _ => {}
        }
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
