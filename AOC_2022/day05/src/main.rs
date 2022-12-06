use regex::Regex;
use std::fs;

const ASCII_UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn init_stacks(contents: &str) -> Vec<Vec<char>> {
    let mut lines = contents.lines().peekable();

    let first_line = lines.peek().cloned().unwrap();

    let number_of_stacks = (first_line.len() + 1) / 4;
    let mut layout: Vec<Vec<char>> = Vec::with_capacity(number_of_stacks as usize);
    for _i in 0..number_of_stacks {
        layout.push(Vec::new());
    }

    for line in lines {
        let line_len = line.len();
        for i in (0..line_len).step_by(4) {
            let diff = match line_len - i {
                x if x < 4 => x,
                _ => 4,
            };

            let chars = &mut line[i..i + diff].chars();
            let box_char = chars.nth(1).unwrap();
            let stack_id: usize = i / 4;
            match ASCII_UPPER.iter().any(|&x| x == box_char) {
                true => layout[stack_id].insert(0, box_char),
                false => continue,
            };
        }
    }

    return layout;
}

fn parse_instruction(line: &str) -> Result<(u16, usize, usize), &str> {
    let re = Regex::new(r"^move (?P<volume>\d+) from (?P<src>\d+) to (?P<dst>\d+)$").unwrap();
    return match re.is_match(line) {
        true => Ok(split_cap(re.captures_iter(line))),
        false => Err("No valid instructions"),
    };
}

fn split_cap(mut captures: regex::CaptureMatches) -> (u16, usize, usize) {
    let cap = captures.next().unwrap();
    let volume = &cap["volume"].parse::<u16>().unwrap();
    let src = &cap["src"].parse::<usize>().unwrap() - 1;
    let dst = &cap["dst"].parse::<usize>().unwrap() - 1;
    return (*volume, src, dst);
}

fn apply_instructions(stacks: &mut Vec<Vec<char>>, contents: &str) {
    let lines = contents.lines();

    for line in lines {
        let (volume, src, dst) = match parse_instruction(line) {
            Ok(n) => n,
            Err(_e) => continue,
        };

        for _i in 0..volume {
            let box_to_move = stacks[src].pop().unwrap();
            stacks[dst].push(box_to_move);
        }
    }
}

fn apply_instructions_2(stacks: &mut Vec<Vec<char>>, contents: &str) {
    let lines = contents.lines();

    for line in lines {
        let (volume, src, dst) = match parse_instruction(line) {
            Ok(n) => n,
            Err(_e) => continue,
        };
        let mut boxes_to_move: Vec<char> = Vec::new();
        for _i in 0..volume {
            let box_to_move = stacks[src].pop().unwrap();
            boxes_to_move.insert(0, box_to_move);
        }
        for _box in boxes_to_move {
            stacks[dst].push(_box);
        }
    }
}

fn main() {
    let fp = if cfg!(debug_assertions) {
        "data/example"
    } else {
        "data/input"
    }
    .to_string();

    let contents = fs::read_to_string(fp).expect("Should be able to read file");

    let mut stacks = init_stacks(&contents);

    apply_instructions(&mut stacks, &contents);

    let mut final_str = "".to_owned();

    for stack in stacks {
        let mut ch: char = '!';
        for c in stack {
            #[cfg(debug_assertions)]
            print!("{c} ");
            ch = c;
        }
        final_str.push(ch);
        #[cfg(debug_assertions)]
        println!("")
    }

    println!("1st Str {final_str}");

    let mut stacks_2 = init_stacks(&contents);
    apply_instructions_2(&mut stacks_2, &contents);

    let mut final_str = "".to_owned();

    for stack in stacks_2 {
        let mut ch: char = '!';
        for c in stack {
            #[cfg(debug_assertions)]
            print!("{c} ");
            ch = c;
        }
        final_str.push(ch);
        #[cfg(debug_assertions)]
        println!("")
    }

    println!("2nd Str {final_str}")
}
