use std::collections::HashSet;
use std::fs;

#[cfg(debug_assertions)]
fn printvisible(set: &HashSet<usize>, width: usize, height: usize) {
    let blankline = ".".repeat(width) + "\n";
    let mut blankgrid = blankline.repeat(height);
    for value in set {
        blankgrid.replace_range(value..&(value + 1), "@");
    }

    println!("{blankgrid}");
}

fn part1(contents: &str) -> String {
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

    #[cfg(debug_assertions)]
    printvisible(&visible, width, height);
    return visible.len().to_string();
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

fn part2(contents: &str) -> String {
    let height: usize = contents.lines().count();
    let width: usize = contents.len() / height - 1;

    let mut best_score = 0;
    for row in 0..height {
        for col in 0..width {
            match score_tree(&contents, (row, col), (height, width)) {
                n if n > best_score => best_score = n,
                _ => {}
            }
        }
    }
    return best_score.to_string();
}

fn main() {
    let fp = if cfg!(debug_assertions) {
        "data/example"
    } else {
        "data/input"
    }
    .to_string();

    let contents = fs::read_to_string(fp).expect("Should be able to read file");

    let result1 = part1(&contents);
    let result2 = part2(&contents);
    println!("Part 1 {result1} Part 2 {result2}")
}
