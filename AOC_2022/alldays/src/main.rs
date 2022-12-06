use regex::Regex;
use std::collections::HashSet;
use std::fs;

fn day01(contents: &str) -> (String, String) {
    #[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
    struct Elf {
        calories: i32,
    }

    fn elf_calories(content: &str) -> Vec<Elf> {
        let lines = content.lines();
        let mut elves: Vec<Elf> = Vec::new();

        let mut active_elf = Elf { calories: 0 };
        for l in lines {
            if l.is_empty() {
                elves.push(active_elf);
                active_elf = Elf { calories: 0 };
                continue;
            }
            active_elf.calories += match l.parse::<i32>() {
                Ok(n) => n,
                Err(_e) => panic!(),
            };
        }
        elves.push(active_elf);

        return elves;
    }
    let mut elves = elf_calories(&contents);

    elves.sort_by_key(|k| -k.calories);

    let part1 = elves[0].calories;

    let mut part2 = 0;

    for i in 0..3 {
        part2 += elves[i].calories;
    }
    return (part1.to_string(), part2.to_string());
}

fn day02(contents: &str) -> (String, String) {
    let mut scores_1: Vec<usize> = Vec::new();
    let mut scores_2: Vec<usize> = Vec::new();
    fn scoring_1(line: &str) -> usize {
        match line {
            "A X" => 4,
            "A Y" => 8,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 7,
            "C Y" => 2,
            "C Z" => 6,
            _ => 0,
        }
    }
    fn scoring_2(line: &str) -> usize {
        match line {
            "A X" => 3,
            "A Y" => 4,
            "A Z" => 8,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 2,
            "C Y" => 6,
            "C Z" => 7,
            _ => 0,
        }
    }

    for l in contents.lines() {
        scores_1.push(scoring_1(&l));
        scores_2.push(scoring_2(&l));
    }
    return (
        scores_1.iter().sum::<usize>().to_string(),
        scores_2.iter().sum::<usize>().to_string(),
    );
}

fn day03(contents: &str) -> (String, String) {
    fn parse_backpack_contents(content: &str) -> Vec<char> {
        let mut extra_items: Vec<char> = Vec::new();
        let lines = content.lines();
        for line in lines {
            let len = line.chars().count() / 2;
            let mut fh = HashSet::new();
            for c in line[0..len].chars() {
                fh.insert(c);
            }
            let mut sh = HashSet::new();
            for c in line[len..].chars() {
                sh.insert(c);
            }

            let diff = fh.intersection(&sh).collect::<Vec<&char>>();

            match diff.len() {
                1 => extra_items.push(*diff[0]),
                _ => panic!(""),
            }
        }
        return extra_items;
    }

    fn find_badge(content: &str) -> Vec<char> {
        let mut badges: Vec<char> = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        for i in (0..lines.len()).step_by(3) {
            let l1: HashSet<char> = HashSet::from_iter(lines[i].chars());
            let l2: HashSet<char> = HashSet::from_iter(lines[i + 1].chars());
            let l3: HashSet<char> = HashSet::from_iter(lines[i + 2].chars());

            let diff1 = l1.intersection(&l2);
            let mut l12 = HashSet::new();
            for d in diff1 {
                l12.insert(*d);
            }
            let diff2 = l3.intersection(&l12).collect::<Vec<&char>>();

            match diff2.len() {
                1 => badges.push(*diff2[0]),
                _ => panic!(""),
            }
        }
        return badges;
    }

    fn prioritise(errors: Vec<char>) -> u32 {
        let mut total_priorities = 0;
        let priorities: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .collect();

        for c in errors {
            let priority = priorities
                .iter()
                .position(|&x| x == c)
                .expect("Can't find {c}");
            total_priorities += priority as u32 + 1;
        }
        return total_priorities;
    }

    let wrong = parse_backpack_contents(&contents);
    let total_score = prioritise(wrong);

    let badges = find_badge(&contents);
    let badge_score = prioritise(badges);
    return (total_score.to_string(), badge_score.to_string());
}

fn day04(contents: &str) -> (String, String) {
    let re = Regex::new(r"^(?P<s1>\d+)-(?P<e1>\d+),(?P<s2>\d+)-(?P<e2>\d+)$").unwrap();

    fn split_regex(mut captures: regex::CaptureMatches) -> (usize, usize, usize, usize) {
        let cap = captures.next().unwrap();
        return (
            cap["s1"].parse::<usize>().unwrap(),
            cap["e1"].parse::<usize>().unwrap(),
            cap["s2"].parse::<usize>().unwrap(),
            cap["e2"].parse::<usize>().unwrap(),
        );
    }

    let mut part_1 = 0;
    let mut part_2 = 0;
    for l in contents.lines() {
        let elf_pos = match re.is_match(l) {
            true => split_regex(re.captures_iter(l)),
            false => continue,
        };
        part_1 += match elf_pos {
            (a, b, x, y) if (a <= x) && (b >= y) => 1,
            (a, b, x, y) if (a >= x) && (b <= y) => 1,
            _ => 0,
        };
        part_2 += match elf_pos {
            (a, b, x, _) if (b >= x) && (a <= x) => 1,
            (a, _, x, y) if (y >= a) && (x <= a) => 1,
            _ => 0,
        };
    }
    return (part_1.to_string(), part_2.to_string());
}

fn day05(contents: &str) -> (String, String) {
    const ASCII_UPPER: [char; 26] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
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

    let mut stacks = init_stacks(&contents);

    apply_instructions(&mut stacks, &contents);

    let mut first_str = "".to_owned();

    for stack in stacks {
        let mut ch: char = '!';
        for c in stack {
            ch = c;
        }
        first_str.push(ch);
    }

    let mut stacks_2 = init_stacks(&contents);
    apply_instructions_2(&mut stacks_2, &contents);

    let mut final_str = "".to_owned();

    for stack in stacks_2 {
        let mut ch: char = '!';
        for c in stack {
            ch = c;
        }
        final_str.push(ch);
    }

    return (first_str, final_str);
}

fn day06(contents: &str) -> (String, String) {
    fn incr_buff_pos(index: usize, buff_sz: usize) -> usize {
        return match index {
            n if n >= buff_sz - 1 => 0,
            _ => index + 1,
        };
    }

    trait Uniq {
        fn is_unique(&self) -> bool;
    }

    impl Uniq for Vec<char> {
        fn is_unique(&self) -> bool {
            let sh: HashSet<&char> = HashSet::from_iter(self);
            let hash_cap = sh.len();
            let my_cap = self.len();

            return hash_cap == my_cap;
        }
    }

    fn find_start(contents: &str, buff_sz: usize) -> Result<usize, &str> {
        let mut rb: Vec<char> = vec!['!'; buff_sz];

        let mut buff_index: usize = 0;
        let mut char_index: usize = 0;

        for _char in contents.chars() {
            rb[buff_index] = _char;
            char_index += 1;
            buff_index = incr_buff_pos(buff_index, buff_sz);

            if (char_index > buff_sz) && (rb.is_unique()) {
                break;
            }
        }
        return Ok(char_index);
    }

    let packet_index = match find_start(&contents, 4) {
        Ok(n) => n,
        Err(i) => panic!("{i}"),
    };

    let message_index = match find_start(&contents, 14) {
        Ok(n) => n,
        Err(i) => panic!("{i}"),
    };
    return (packet_index.to_string(), message_index.to_string());
}

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

        let (soln1, soln2) = match i {
            1 => day01(&contents),
            2 => day02(&contents),
            3 => day03(&contents),
            4 => day04(&contents),
            5 => day05(&contents),
            6 => day06(&contents),
            _ => continue,
        };

        println!("Day {i:02}\t Part 1: {soln1:<16} Part 2: {soln2:<16}");
    }
}
