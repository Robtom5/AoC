use std::fs;

fn part1(contents: &str) -> String {
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
            _n if _n <= 1 => print!("##"),
            _ => print!(".."),
        }
        match word.parse::<i32>() {
            Ok(n) => register += n,
            Err(_) => {}
        }
        if (cycle % 40) == 0 {
            println!("");
        }
    }
    return strength_sum.to_string();
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
    println!("part 1 {res1}")
}
