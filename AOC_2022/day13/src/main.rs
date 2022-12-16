use std::cmp::Ordering;
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

fn part2(contents: &str) -> String {
    let appended_contents = format!("{}{}", contents, "\n[[2]]\n[[6]]\n");
    let packets = appended_contents.split("\n\n");

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

        all_packets.push(l1_vec);
        all_packets.push(l2_vec);
    }

    all_packets.sort_by(|a, b| order_lists(a, b));

    #[cfg(debug_assertions)]
    for n in all_packets.iter() {
        println!("{}", vec_to_string(&n))
    }

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

    (packet_1 * packet_2).to_string()
}

fn order_lists(left_list: &Vec<ListItem>, right_list: &Vec<ListItem>) -> std::cmp::Ordering {
    match compare_lists(left_list, right_list) {
        Some(true) => Ordering::Less,
        Some(false) => Ordering::Greater,
        None => Ordering::Equal,
    }
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

#[allow(dead_code)]
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

fn main() {
    let fp = if cfg!(debug_assertions) {
        "data/example"
    } else {
        "data/input"
    }
    .to_string();

    let contents = fs::read_to_string(fp).expect("Should be able to read file");

    let res1 = part1(&contents);
    let res2 = part2(&contents);

    println!("Part 1 {res1} Part 2 {res2}");
}
