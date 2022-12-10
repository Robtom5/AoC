use core::hash::Hash;
use core::hash::Hasher;
use itertools::Itertools;
use regex::Regex;
use std::cmp::max;
use std::cmp::min;
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
            return self.len() == self.into_iter().unique().count();
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

fn day07(contents: &str) -> (String, String) {
    fn relevant_sz((name, sz): &(String, u64), start: &str) -> u64 {
        match name.starts_with(start) {
            true => return *sz,
            false => return 0,
        }
    }
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

fn day08(contents: &str) -> (String, String) {
    let height: usize = contents.lines().count();
    let width: usize = contents.len() / height - 1; // n lines with n carries

    let mut visible = HashSet::new();

    for row in 0..height {
        // Count rowwise
        let row_index = row * (width + 1);
        let mut prev_height_l: i16 = -1;
        let mut prev_height_r: i16 = -1;

        for col in 0..width {
            let l_index = row_index + col;
            let l_height = match contents.get(l_index..l_index + 1) {
                Some(n) => n.parse::<usize>().unwrap(),
                None => 0,
            };

            let r_index = row_index + (width - 1 - col);
            let r_height = match contents.get(r_index..r_index + 1) {
                Some(n) => n.parse::<usize>().unwrap(),
                None => 0,
            };

            match l_height {
                h if h as i16 > prev_height_l => {
                    visible.insert(l_index);
                    prev_height_l = h as i16;
                }
                _ => {}
            }

            match r_height {
                h if h as i16 > prev_height_r => {
                    visible.insert(r_index);

                    prev_height_r = h as i16;
                }
                _ => {}
            }
        }
    }

    for col in 0..width {
        let mut prev_height_u: i16 = -1;
        let mut prev_height_d: i16 = -1;

        for row in 0..height {
            let u_index = row * (width + 1) + col;

            let u_height = match contents.get(u_index..u_index + 1) {
                Some(n) => n.parse::<usize>().unwrap(),
                None => 0,
            };

            let d_index = (height - row - 1) * (width + 1) + col;
            let d_height = match contents.get(d_index..d_index + 1) {
                Some(n) => n.parse::<usize>().unwrap(),
                None => 0,
            };

            match u_height {
                h if h as i16 > prev_height_u => {
                    visible.insert(u_index);
                    prev_height_u = h as i16;
                }
                _ => {}
            }

            match d_height {
                h if h as i16 > prev_height_d => {
                    visible.insert(d_index);

                    prev_height_d = h as i16;
                }
                _ => {}
            }
        }
    }
    fn score_tree(
        contents: &str,
        (row, col): (usize, usize),
        (height, width): (usize, usize),
    ) -> usize {
        let index = row * (width + 1) + col;
        let tree_height = contents
            .get(index..index + 1)
            .expect("Invalid index")
            .parse::<usize>()
            .unwrap();

        let mut visible_r = 0;
        let mut visible_l = 0;
        let mut visible_u = 0;
        let mut visible_d = 0;

        for x in (col + 1)..width {
            let look_r = row * (width + 1) + x;

            match contents
                .get(look_r..look_r + 1)
                .expect("Invalid index")
                .parse::<usize>()
                .unwrap()
            {
                n if n >= tree_height => {
                    visible_r += 1;
                    break;
                }
                _ => visible_r += 1,
            }
        }

        for x in (0..col).rev() {
            let look_l = row * (width + 1) + x;

            match contents
                .get(look_l..look_l + 1)
                .expect("Invalid index")
                .parse::<usize>()
                .unwrap()
            {
                n if n >= tree_height => {
                    visible_l += 1;
                    break;
                }
                _ => visible_l += 1,
            }
        }

        for y in (row + 1)..height {
            let look_d = y * (width + 1) + col;

            match contents
                .get(look_d..look_d + 1)
                .expect("Invalid index")
                .parse::<usize>()
                .unwrap()
            {
                n if n >= tree_height => {
                    visible_d += 1;
                    break;
                }
                _ => visible_d += 1,
            }
        }

        for y in (0..row).rev() {
            let look_u = y * (width + 1) + col;

            match contents
                .get(look_u..look_u + 1)
                .expect("Invalid index")
                .parse::<usize>()
                .unwrap()
            {
                n if n >= tree_height => {
                    visible_u += 1;
                    break;
                }
                _ => visible_u += 1,
            }
        }
        return visible_r * visible_l * visible_u * visible_d;
    }

    let mut best_score = 0;
    for row in 0..height {
        for col in 0..width {
            match score_tree(&contents, (row, col), (height, width)) {
                n if n > best_score => best_score = n,
                _ => {}
            }
        }
    }

    (visible.len().to_string(), best_score.to_string())
}

fn day09(contents: &str) -> (String, String) {
    #[derive(Eq)]
    struct Point {
        x: i16,
        y: i16,
    }

    impl PartialEq for Point {
        fn eq(&self, other: &Self) -> bool {
            (self.x == other.x) && (self.y == other.y)
        }
    }

    impl Hash for Point {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.x.hash(state);
            self.y.hash(state);
        }
    }

    struct Rope {
        head: Point,
        tail: Point,
    }

    fn create_point() -> Point {
        Point { x: 0, y: 0 }
    }

    fn create_point_from_coords(x: i16, y: i16) -> Point {
        Point { x: x, y: y }
    }

    fn create_rope() -> Rope {
        Rope {
            head: create_point(),
            tail: create_point(),
        }
    }

    trait RopePhysics {
        fn update_tail(&mut self) -> Point;
        fn update_head(&mut self, pt: &Point);
    }

    trait Distance<Rhs = Self> {
        fn distance_to(&self, other: &Rhs) -> i16;
    }
    impl RopePhysics for Rope {
        fn update_tail(&mut self) -> Point {
            let Point { x: hx, y: hy } = self.head;

            let (dx, dy): (i16, i16) = match self.tail.distance_to(&self.head) {
                _n if _n > 1 => {
                    match &self.tail {
                        Point { x, y } if x == &hx => (*x, y + (hy - y).signum()), // Same row
                        Point { x, y } if y == &hy => (x + (hx - x).signum(), *y), //Same column,
                        p => (p.x + (hx - p.x).signum(), p.y + (hy - p.y).signum()), // Point { x, y } => (*x, *y),
                    }
                }
                _ => (self.tail.x, self.tail.y),
            };
            self.tail.x = dx;
            self.tail.y = dy;
            create_point_from_coords(dx, dy)
        }

        fn update_head(&mut self, pt: &Point) {
            self.head.x = pt.x;
            self.head.y = pt.y;
        }
    }

    impl Distance for Point {
        fn distance_to(&self, other: &Point) -> i16 {
            let dx = (self.x - other.x).abs();
            let dy = (self.y - other.y).abs();

            return max(dx, dy);
        }
    }

    fn rope_behaviour(contents: &str, rope_len: usize) -> String {
        let mut visited: HashSet<Point> = HashSet::new();

        visited.insert(create_point());

        let mut ropes: Vec<Rope> = Vec::with_capacity(rope_len);
        for _i in 0..rope_len {
            ropes.push(create_rope());
        }

        for line in contents.lines() {
            let (dir, dist) = match line.split_once(" ") {
                Some(n) => n,
                None => continue,
            };
            let dist_val = match dist.parse::<usize>() {
                Ok(n) => n,
                Err(_) => continue,
            };

            // let mut rope = &ropes[0];
            // let mut tail;

            for _i in 0..dist_val {
                let mut rope_iter = ropes.iter_mut();
                let mut rope = rope_iter.next().unwrap();
                match dir {
                    "R" => rope.head.x += 1,
                    "L" => rope.head.x -= 1,
                    "U" => rope.head.y += 1,
                    "D" => rope.head.y -= 1,
                    _ => panic!("Unknown direction"),
                };

                let mut last_tail = rope.update_tail();
                for r in rope_iter {
                    r.update_head(&last_tail);
                    last_tail = r.update_tail();
                }
                visited.insert(last_tail);
            }
        }
        visited.len().to_string()
    }

    (rope_behaviour(&contents, 1), rope_behaviour(&contents, 9))
}

fn day10(contents: &str) -> (String, String) {
    let mut cycle: i32 = 0;
    let mut strength_sum = 0;
    let mut register: i32 = 1;

    for word in contents.split_whitespace() {
        cycle += 1;

        match cycle {
            c if (c + 20) % 40 == 0 => {
                let strength = register * cycle;
                strength_sum += strength;
            }
            _ => {}
        }

        match (register - ((cycle % 40) - 1)).abs() {
            _n if _n <= 1 => print!("██"),
            _ => print!("  "),
        }
        match word.parse::<i32>() {
            Ok(n) => register += n,
            Err(_) => {}
        }
        if (cycle % 40) == 0 {
            println!("");
        }
    }

    return (strength_sum.to_string(), "⬆".to_string());
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

        let (_soln1, _soln2) = match i {
            1 => day01(&contents),
            2 => day02(&contents),
            3 => day03(&contents),
            4 => day04(&contents),
            5 => day05(&contents),
            6 => day06(&contents),
            7 => day07(&contents),
            8 => day08(&contents),
            9 => day09(&contents),
            10 => day10(&contents),
            _ => continue,
        };

        println!("Day {i:02}\t Part 1: {_soln1:<16} Part 2: {_soln2:<16}");
    }
}
