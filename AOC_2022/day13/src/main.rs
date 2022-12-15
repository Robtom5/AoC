use std::fs;

use std::str::Split;

#[derive(Debug)]
enum ListItem {
    Number(u32),
    Vec(Vec<ListItem>),
}

fn part1(contents: &str) -> String {
    let packets = contents.split("\n\n");

    let mut index = 1;
    let mut index_sum = 0;

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

        #[cfg(debug_assertions)]
        {
            print_out(&l1_vec);
            println!("");
            print_out(&l2_vec);
            println!("");
            println!("");
        }

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
    }

    index_sum.to_string()
}

#[allow(dead_code)]
fn print_out(list: &Vec<ListItem>) {
    print!("[ ");
    for n in list {
        match n {
            ListItem::Number(x) => print!("{x}"),
            ListItem::Vec(y) => print_out(&y),
        }
        print!(" ")
    }
    print!("]");
}

fn compare_lists(left_list: &Vec<ListItem>, right_list: &Vec<ListItem>) -> Option<bool> {
    let mut l_iter = left_list.iter();
    let mut r_iter = right_list.iter();
    // print_out(left_list);
    // println!("");
    // print_out(right_list);
    // println!("");
    // println!("");
    loop {
        let (l, r) = match (l_iter.next(), r_iter.next()) {
            (Some(n), Some(m)) => (n, m),
            (Some(_n), None) => return Some(false), // Right ran out first
            (None, Some(_m)) => return Some(true),  // Left ran out first // This might not be true
            (None, None) => return None,
        };

        // Failure occuring at reading 39. One tip is to do a for instead of while (expand all pairs)
        // That said, reading 40 is not parsing correctly

        // [[1,[2,[10,8,2,1,1]],0]]
        // [[[1]],[[[2,4,10,2],[]],3,8],[9,3,[5,[3,0],[0],[4]],6,[[9,8,3,7],4,[10,10,8],10,[6,6]]],[[[3],7,[],[10,5]],0],[5,[[3,9,0,2,1],0,[4,5,2],[6]]]]

        // [[[]],[[[8,9,10,8],[6,5,4,10,10],[8,10,0,2,0],[1,7,1],[]],[[]],7]]
        // [[],[],[8,10]]

        // Behaviour when both out needs some work
        // need to handle when both out

        // It hasnt parsed the second line correctly

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

fn main() {
    let fp = if cfg!(debug_assertions) {
        "data/example"
    } else {
        "data/input"
    }
    .to_string();

    let contents = fs::read_to_string(fp).expect("Should be able to read file");

    let res1 = part1(&contents);

    println!("Part 1 {res1}");
}
