use core::str::FromStr;

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

pub fn day11(contents: &str) -> (String, String) {
    let res1 = part1(&contents);
    let res2 = part2(&contents);
    (res1, res2)
}
