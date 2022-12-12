use crate::Regex;

pub fn day01(contents: &str) -> (String, String) {
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

pub fn day02(contents: &str) -> (String, String) {
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

pub fn day03(contents: &str) -> (String, String) {
    trait BitEncode {
        fn to_bit(&self) -> u64;
    }

    impl BitEncode for char {
        fn to_bit(&self) -> u64 {
            return match self.is_ascii_uppercase() {
                true => self.to_digit(36).unwrap() + 17,
                false => self.to_digit(36).unwrap() - 9,
            } as u64;
        }
    }

    impl BitEncode for str {
        fn to_bit(&self) -> u64 {
            let mut bit: u64 = 0b0;
            for c in self.chars() {
                bit |= 1 << c.to_bit();
            }
            return bit;
        }
    }

    fn find_duplicates(content: &str) -> u32 {
        let mut priority_sum = 0;
        let lines = content.lines();

        for line in lines {
            let len = line.chars().count() / 2;

            let left = line[0..len].to_bit();
            let right = line[len..].to_bit();

            let diff: u64 = left & right;

            priority_sum += match diff & diff - 1 {
                0 => diff.ilog2(),
                _ => panic!(""),
            }
        }
        return priority_sum;
    }

    fn find_badge(content: &str) -> u32 {
        let mut badges_sum = 0;
        let lines: Vec<&str> = content.lines().collect();
        for i in (0..lines.len()).step_by(3) {
            let diff: u64 = lines[i].to_bit() & lines[i + 1].to_bit() & lines[i + 2].to_bit();
            badges_sum += match diff & diff - 1 {
                0 => diff.ilog2(),
                _ => panic!(""),
            }
        }
        badges_sum
    }

    let total_score = find_duplicates(&contents);
    let badge_score = find_badge(&contents);

    return (total_score.to_string(), badge_score.to_string());
}

pub fn day04(contents: &str) -> (String, String) {
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

pub fn day05(contents: &str) -> (String, String) {
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
