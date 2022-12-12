use core::str::FromStr;
use pathfinding::prelude::bfs;

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

pub fn day12(contents: &str) -> (String, String) {
    const START_CHAR: char = 'S';
    const TARGET_CHAR: char = 'E';

    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct Pos(usize, usize);

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
