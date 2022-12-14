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

        load_vec(line_1.next().unwrap(), &mut l1_vec, &mut line_1);
        load_vec(line_2.next().unwrap(), &mut l2_vec, &mut line_2);

        println!("{:?}", l1_vec);
        println!("{:?}", l2_vec);
        println!()
    }

    "".to_string()
}

fn load_vec(
    first_elem: &str,
    vector_to_load: &mut Vec<ListItem>,
    string_iter: &mut Split<'_, &str>,
) {
    match first_elem.starts_with("[") {
        true => {
            let mut nested = vec![];

            load_vec(&first_elem[1..], &mut nested, string_iter);
            vector_to_load.push(ListItem::Vec(nested))
        }
        false => {
            let x: &[_] = &['[', ']'];
            match first_elem.trim_matches(x).parse::<u32>() {
                Ok(n) => vector_to_load.push(ListItem::Number(n)),
                Err(_) => return, // no number to return a level
            };
        }
    }
    match first_elem.ends_with("]") {
        true => return,
        false => {}
    }
    loop {
        let next_entry = match string_iter.next() {
            Some(n) => n,
            None => break,
        };
        match next_entry.starts_with("[") {
            true => {
                let mut nested = vec![];
                load_vec(&next_entry[1..], &mut nested, string_iter);
                vector_to_load.push(ListItem::Vec(nested))
            }
            false => match next_entry.trim_end_matches("]").parse::<u32>() {
                Ok(n) => vector_to_load.push(ListItem::Number(n)),
                Err(_) => {}
            },
        }
        match next_entry.ends_with("]") {
            true => break,
            false => {}
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

    let res1 = part1(&contents);

    println!("Part 1 {res1}");
}
