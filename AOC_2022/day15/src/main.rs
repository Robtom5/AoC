use std::fs;

fn main() {
    let fp = if cfg!(debug_assertions) {
        "data/example"
    } else {
        "data/input"
    }
    .to_string();

    let contents = fs::read_to_string(fp).expect("Should be able to read file");

    // Pseudo code part1 => for each sensor work out it's manhatten distance to nearest beacon. Then each value that is ruled out
}
