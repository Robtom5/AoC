use core::str::FromStr;
use pathfinding::prelude::bfs;
use std::cmp::Ordering;
use std::str::Split;

pub fn day11(contents: &str) -> (String, String) {
    enum Operator {
        Add,
        Mul,
        Exp,
    }

    impl FromStr for Operator {
        type Err = ();

        fn from_str(input: &str) -> Result<Operator, Self::Err> {
            match input {
                "+" => Ok(Operator::Add),
                "*" => Ok(Operator::Mul),
                "^" => Ok(Operator::Exp),
                _ => Err(()),
            }
        }
    }

    struct Monkey {
        #[allow(dead_code)]
        index: usize,
        items: Vec<i128>,
        inspections: u64,
        stress_op: Operator,
        stress_fac: u32,
        target_test: i128,
        target_true: usize,
        target_false: usize,
    }

    impl Monkey {
        fn new(raw_text: &str) -> Monkey {
            let mut lines = raw_text.lines();
            let monkey_index = lines
                .next()
                .expect("Not enough lines to make a monkey (index)")[7..8]
                .parse::<usize>()
                .expect("Can't find monkey index"); // Discard monkey name
            let items_line = &lines.next().unwrap()[18..];
            let op_line = &mut lines.next().unwrap().rsplit(" ");
            let tval = lines
                .next()
                .expect("Not enough lines to make a monkey (tval)")
                .rsplit_once(" ")
                .unwrap()
                .1
                .parse::<i128>()
                .expect("Can't find test value");
            let ttrue = lines
                .next()
                .expect("Not enough lines to make a monkey (ttrue)")
                .rsplit_once(" ")
                .unwrap()
                .1
                .parse::<usize>()
                .expect("Can't find target if true");
            let tfalse = lines
                .next()
                .expect("Not enough lines to make a monkey (tfalse)")
                .rsplit_once(" ")
                .unwrap()
                .1
                .parse::<usize>()
                .expect("Can't find target if false");

            let (opp, fac) = match op_line.next().unwrap() {
                "old" => (Operator::Exp, 2),
                n => (
                    Operator::from_str(op_line.next().unwrap()).unwrap(),
                    n.parse::<u32>().unwrap(),
                ),
            };

            let mut monkey = Monkey {
                index: monkey_index,
                items: Vec::new(),
                inspections: 0,
                stress_op: opp,
                stress_fac: fac,
                target_test: tval,
                target_true: ttrue,
                target_false: tfalse,
            };

            for num in items_line.split(", ") {
                monkey
                    .items
                    .push(num.parse::<i128>().expect("Couldn't parse item"));
            }

            monkey
        }

        fn catch(&mut self, new: i128) {
            self.items.push(new);
        }

        fn examine(&mut self) -> Option<i128> {
            match self.items.pop() {
                Some(old) => {
                    self.inspections += 1;
                    Some(match self.stress_op {
                        Operator::Add => old + self.stress_fac as i128,
                        Operator::Mul => old * self.stress_fac as i128,
                        Operator::Exp => i128::pow(old, self.stress_fac),
                    })
                }
                None => None,
            }
        }

        fn throw(&self, new: i128) -> usize {
            match new % self.target_test {
                0 => self.target_true,
                _ => self.target_false,
            }
        }
    }

    fn load_monkeys(contents: &str) -> Vec<Monkey> {
        let mut monkeys = Vec::new();

        for block in contents.split("\n\n") {
            monkeys.push(Monkey::new(block));
        }

        monkeys
    }

    fn part1(contents: &str) -> String {
        let mut monkeys = load_monkeys(contents);
        let _num_monkeys = monkeys.len();
        for _round in 1..=20 {
            for i in 0..monkeys.len() {
                let mut monkey = monkeys.remove(i);
                let num_items = monkey.items.len();
                for _ in 0..num_items {
                    let (target, new) = match monkey.examine() {
                        Some(new) => {
                            let calmer_new = new / 3;
                            let target = monkey.throw(calmer_new);
                            (target, calmer_new)
                        }
                        None => break,
                    };
                    let target_index = match i > target {
                        true => target,
                        false => target - 1,
                    };
                    let mut tm = monkeys.remove(target_index);
                    tm.catch(new);
                    monkeys.insert(target_index, tm);
                }
                monkeys.insert(i, monkey);
            }
        }

        monkeys.sort_by(|a, b| b.inspections.partial_cmp(&a.inspections).unwrap());

        let monkey_business = monkeys[0].inspections * monkeys[1].inspections;

        monkey_business.to_string()
    }

    fn part2(contents: &str) -> String {
        let mut monkeys = load_monkeys(contents);
        let _num_monkeys = monkeys.len();
        let mut max_modulo = 1;
        monkeys.iter().for_each(|m| max_modulo *= m.target_test);
        for _round in 1..=10000 {
            for i in 0..monkeys.len() {
                let mut monkey = monkeys.remove(i);
                let num_items = monkey.items.len();
                for _ in 0..num_items {
                    let (target, new) = match monkey.examine() {
                        Some(new) => {
                            let bound_new = new % max_modulo;
                            let target = monkey.throw(bound_new);
                            (target, bound_new)
                        }
                        None => break,
                    };
                    let target_index = match i > target {
                        true => target,
                        false => target - 1,
                    };
                    let mut tm = monkeys.remove(target_index);
                    tm.catch(new);
                    monkeys.insert(target_index, tm);
                }
                monkeys.insert(i, monkey);
            }
        }

        monkeys.sort_by(|a, b| b.inspections.partial_cmp(&a.inspections).unwrap());

        let monkey_business = monkeys[0].inspections * monkeys[1].inspections;

        monkey_business.to_string()
    }
    let res1 = part1(&contents);
    let res2 = part2(&contents);
    (res1, res2)
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

pub fn day12(contents: &str) -> (String, String) {
    const START_CHAR: char = 'S';
    const TARGET_CHAR: char = 'E';

    impl Pos {
        fn successors(&self, grid: &Vec<Vec<u8>>, grid_sz: (usize, usize)) -> Vec<Pos> {
            let &Pos(x, y) = self;
            let pos_value = grid[y][x];
            let neighbors = Pos::neighbors(x, y, grid_sz);
            let mut successors: Vec<Pos> = Vec::with_capacity(neighbors.len());
            for (dx, dy) in neighbors {
                let s_v = grid[dy][dx];
                match s_v as i16 - pos_value as i16 {
                    diff if diff <= 1 => {
                        successors.push(Pos(dx as usize, dy as usize));
                    }
                    _ => {}
                };
            }
            successors
        }

        fn successors_inv(&self, grid: &Vec<Vec<u8>>, grid_sz: (usize, usize)) -> Vec<Pos> {
            let &Pos(x, y) = self;
            let pos_value = grid[y][x];
            let neighbors = Pos::neighbors(x, y, grid_sz);
            let mut successors: Vec<Pos> = Vec::with_capacity(neighbors.len());
            for (dx, dy) in neighbors {
                let s_v = grid[dy][dx];
                match pos_value as i16 - s_v as i16 {
                    diff if diff <= 1 => {
                        successors.push(Pos(dx as usize, dy as usize));
                    }
                    _ => {}
                };
            }
            successors
        }

        fn neighbors(x: usize, y: usize, (width, height): (usize, usize)) -> Vec<(usize, usize)> {
            match (x, y) {
                (0, 0) => vec![(1, 0), (0, 1)],                      // TL
                (0, y) if (y == height) => vec![(1, y), (0, y - 1)], // BL
                (x, 0) if (x == width) => vec![(x - 1, 0), (x, 1)],  // TR
                (x, y) if (x == width) && (y == height) => vec![(x - 1, y), (x, y - 1)], // BR
                (0, y) => vec![(0, y + 1), (0, y - 1), (1, y)],      // L
                (x, 0) => vec![(x + 1, 0), (x - 1, 0), (x, 1)],      // T
                (x, y) if y == height => vec![(x + 1, y), (x - 1, y), (x, y - 1)], // B
                (x, y) if x == width => vec![(x, y + 1), (x, y - 1), (x - 1, y)], // R
                (x, y) => vec![(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)], // C
            }
        }
    }

    fn load_map(contents: &str) -> (Vec<Vec<u8>>, Pos, Pos, (usize, usize)) {
        let height: usize = contents.lines().count();
        let width: usize = contents.len() / height - 1; // n lines with n carries

        let mut grid = vec![vec![0u8; width]; height];
        let mut src: Pos = Pos(0, 0);
        let mut dst: Pos = Pos(0, 0);

        for y in 0..height {
            for x in 0..width {
                let index = ((width + 1) * y) + x;
                let value: char = match contents.chars().nth(index).unwrap() {
                    START_CHAR => {
                        src = Pos(x, y);
                        'a'
                    }
                    TARGET_CHAR => {
                        dst = Pos(x, y);
                        'z'
                    }
                    n => n,
                };
                let v_h = value.to_digit(36).unwrap() - 10;
                grid[y][x] = v_h as u8;
            }
        }
        (grid, src, dst, (width - 1, height - 1))
    }

    let (map, src, dst, sz) = load_map(&contents);

    let result = bfs(&src, |p| p.successors(&map, sz), |p| *p == dst);

    let unwrapped = result.expect("no path found");
    let unwrapped_len = unwrapped.len();

    let pos_result = bfs(&dst, |p| p.successors_inv(&map, sz), |x| map[x.1][x.0] == 1);
    let min_dis = pos_result.unwrap().len();

    return ((unwrapped_len - 1).to_string(), min_dis.to_string());
}

#[derive(Debug)]
enum ListItem {
    Number(u32),
    Vec(Vec<ListItem>),
}

fn order_lists(left_list: &Vec<ListItem>, right_list: &Vec<ListItem>) -> std::cmp::Ordering {
    match compare_lists(left_list, right_list) {
        Some(true) => Ordering::Less,
        Some(false) => Ordering::Greater,
        None => Ordering::Equal,
    }
}

fn vec_to_string(list: &Vec<ListItem>) -> String {
    let mut str_rep = "[".to_owned();
    for n in list {
        match n {
            ListItem::Number(x) => str_rep.push_str(&x.to_string()),
            ListItem::Vec(y) => str_rep.push_str(&vec_to_string(&y)),
        }
        str_rep.push(',')
    }
    // print!("]");
    if str_rep.ends_with(',') {
        str_rep = str_rep[..(str_rep.len() - 1)].to_string();
    }
    str_rep.push(']');
    str_rep
}

fn compare_lists(left_list: &Vec<ListItem>, right_list: &Vec<ListItem>) -> Option<bool> {
    let mut l_iter = left_list.iter();
    let mut r_iter = right_list.iter();

    loop {
        let (l, r) = match (l_iter.next(), r_iter.next()) {
            (Some(n), Some(m)) => (n, m),
            (Some(_n), None) => return Some(false), // Right ran out first
            (None, Some(_m)) => return Some(true),  // Left ran out first // This might not be true
            (None, None) => return None,
        };

        match (l, r) {
            (ListItem::Number(x), ListItem::Number(y)) => match (*y as i32) - (*x as i32) {
                0 => {}
                n if n > 0 => return Some(true),
                n if n < 0 => return Some(false),
                _ => {}
            },
            (ListItem::Vec(x), ListItem::Number(y)) => {
                match compare_lists(&x, &vec![ListItem::Number(*y)]) {
                    Some(n) => return Some(n),
                    None => {}
                }
            }
            (ListItem::Number(x), ListItem::Vec(y)) => {
                match compare_lists(&vec![ListItem::Number(*x)], &y) {
                    Some(n) => return Some(n),
                    None => {}
                }
            }

            (ListItem::Vec(x), ListItem::Vec(y)) => match compare_lists(&x, &y) {
                Some(n) => return Some(n),
                None => {}
            },
        }
    }
}

fn load_vec_r(
    first_elem: &str,
    vector_to_load: &mut Vec<ListItem>,
    string_iter: &mut Split<'_, &str>,
) -> Option<String> {
    let mut next_elem = first_elem;
    loop {
        let step_res: Option<String> = match next_elem.starts_with("[") {
            true => {
                let mut nested = vec![];
                let stripped_elem = &next_elem[1..];
                let next_r = load_vec_r(stripped_elem, &mut nested, string_iter);
                vector_to_load.push(ListItem::Vec(nested));

                match next_r {
                    Some(n) => match n.ends_with("]") {
                        true => Some(n[..(n.len() - 1)].to_string()),
                        false => None,
                    },
                    None => None,
                }
            }
            false => {
                match next_elem.ends_with("]") {
                    true => {
                        match next_elem.trim_end_matches("]").parse::<u32>() {
                            Ok(n) => vector_to_load.push(ListItem::Number(n)),
                            Err(_) => {}
                        }
                        return Some(next_elem.to_string());
                    }
                    false => match next_elem.parse::<u32>() {
                        Ok(n) => vector_to_load.push(ListItem::Number(n)),
                        Err(_) => panic!("no data"),
                    },
                }
                None
            }
        };

        match step_res {
            None => {}
            Some(n) => match n.ends_with("]") {
                false => {}
                true => return Some(n),
            },
        }

        next_elem = match string_iter.next() {
            Some(n) => n,
            None => break,
        };
    }
    None
}

pub fn day13(contents: &str) -> (String, String) {
    let appended_contents = format!("{}{}", contents, "\n[[6]]\n[[2]]\n");
    let packets = appended_contents.split("\n\n");

    let mut index = 1;
    let mut index_sum = 0;
    let mut all_packets: Vec<Vec<ListItem>> = Vec::new();
    for packet in packets {
        let mut p_iter = packet.split("\n");
        let mut line_1 = p_iter.next().expect("No left packet found").split(",");
        let mut line_2 = p_iter.next().expect("No right packet found").split(",");

        let mut l1_vec: Vec<ListItem> = Vec::new();
        let mut l2_vec: Vec<ListItem> = Vec::new();

        let first_elem_1 = &line_1.next().unwrap()[1..];
        let first_elem_2 = &line_2.next().unwrap()[1..];

        load_vec_r(first_elem_1, &mut l1_vec, &mut line_1);
        load_vec_r(first_elem_2, &mut l2_vec, &mut line_2);

        match compare_lists(&l1_vec, &l2_vec) {
            Some(true) => {
                index_sum += index;
            }
            Some(false) => {}
            None => {
                println!("{index} {:?} {:?}", l1_vec, l2_vec);
                panic!("Neither valid nor invalid")
            }
        }

        index += 1;

        all_packets.push(l1_vec);
        all_packets.push(l2_vec);
    }
    all_packets.sort_by(|a, b| order_lists(a, b));
    let packet_1 = match all_packets
        .iter()
        .position(|x| vec_to_string(&x) == "[[2]]")
    {
        Some(n) => n + 1,
        None => panic!("Lost first divider packet"),
    };

    let packet_2 = match all_packets
        .iter()
        .position(|x| vec_to_string(&x) == "[[6]]")
    {
        Some(n) => n + 1,
        None => panic!("Lost second divider packet"),
    };

    (index_sum.to_string(), (packet_1 * packet_2).to_string())
}
