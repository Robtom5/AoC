use core::cmp::min;
use pathfinding::prelude::bfs;
use std::fs;
use std::{thread, time};

const START_CHAR: char = 'S';
const TARGET_CHAR: char = 'E';

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn successors(&self, grid: &Vec<Vec<u8>>) -> Vec<Pos> {
        let &Pos(x, y) = self;
        let pos_value = grid[y][x];
        let (height, width) = (grid.len() - 1, grid[0].len() - 1);
        let mut successors: Vec<Pos> = Vec::new();
        let neighbors = match (x, y) {
            (0, 0) => vec![(1, 0), (0, 1)],                      // TL
            (0, y) if (y == height) => vec![(1, y), (0, y - 1)], // BL
            (x, 0) if (x == width) => vec![(x - 1, 0), (x, 1)],  // TR
            (x, y) if (x == width) && (y == height) => vec![(x - 1, y), (x, y - 1)], // BR
            (0, y) => vec![(0, y + 1), (0, y - 1), (1, y)],      // L
            (x, 0) => vec![(x + 1, 0), (x - 1, 0), (x, 1)],      // T
            (x, y) if y == height => vec![(x + 1, y), (x - 1, y), (x, y - 1)], // B
            (x, y) if x == width => vec![(x, y + 1), (x, y - 1), (x - 1, y)], // R
            (x, y) => vec![(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)], // C
        };
        // println!("{dx} {dy} {height} {width}");
        // println!("{x} {y}  {width} {height}  {:?}", neighbors);
        for (dx, dy) in neighbors {
            // let (X, Y) = (x as i16 + dx, y as i16 + dy);
            // print_pt(&Pos(dx, dy), '.', 1);
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
}

fn part1(contents: &str) -> (String, String) {
    let (map, src, dst, starts) = load_map(&contents);

    println!("{:?} {:?}", src, dst);

    let (height, width) = (map.len() - 1, map[0].len() - 1);

    print_grid(width, height);

    let result = bfs(&src, |p| p.successors(&map), |p| *p == dst);

    let unwrapped = result.expect("no path found");
    let unwrapped_len = unwrapped.len() - 1;
    for node in unwrapped {
        print_pt(&node, 'x', 10);
    }
    let mut min_dis = unwrapped_len;
    for start in starts {
        print_grid(width, height);

        let pos_result = bfs(&start, |p| p.successors(&map), |p| *p == dst);
        let pos_len = match pos_result {
            Some(n) => n.len() - 1,
            None => continue,
        };
        min_dis = min(min_dis, pos_len);
    }
    // println!("{esc}[43E", esc = 27 as char);
    return ((unwrapped_len).to_string(), min_dis.to_string());
}

fn print_grid(width: usize, height: usize) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clear
    let blankline = " ".repeat(width as usize) + "\n";
    let blankgrid = blankline.repeat(height as usize);

    println!("{blankgrid}");
}

fn print_pt(point: &Pos, icon: char, interval: u64) {
    let &Pos(x, y) = point;
    print!(
        "{esc}[H{esc}[{y};{x}H{icon}{esc}[H{esc}[43E",
        esc = 27 as char
    );
    // thread::sleep(time::Duration::from_micros(interval));
    thread::sleep(time::Duration::from_micros(interval));
}

// fn printvisible(pt: Pos, width: i16, height: i16) {
//     println!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clear
//     let blankline = ".".repeat(width as usize) + "\n";
//     let mut blankgrid = blankline.repeat(height as usize);
//     let origin = (height / 2) * (width + 1) + (width / 2);
//     for pt in set {
//         let x_val = pt.x;
//         let y_val = pt.y;
//         let value = (y_val * (width + 1) + x_val + origin) as usize;
//         blankgrid.replace_range(value..(value + 1), "@");
//     }
//     println!("{blankgrid}");
// }

fn load_map(contents: &str) -> (Vec<Vec<u8>>, Pos, Pos, Vec<Pos>) {
    let height: usize = contents.lines().count();
    let width: usize = contents.len() / height - 1; // n lines with n carries

    // Final 2d array `&mut [&mut [_]]`
    let mut grid = vec![vec![0u8; width]; height];
    let mut src: Pos = Pos(0, 0);
    let mut dst: Pos = Pos(0, 0);
    let mut starts: Vec<Pos> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let index = ((width + 1) * y) + x;
            let value: char = match contents.chars().nth(index).unwrap() {
                'a' => {
                    starts.push(Pos(x, y));
                    'a'
                }
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
    (grid, src, dst, starts)
}

fn main() {
    let fp = if cfg!(debug_assertions) {
        "data/example"
    } else {
        "data/input"
    }
    .to_string();

    let contents = fs::read_to_string(fp).expect("Should be able to read file");

    let (res1, res2) = part1(&contents);

    println!("Part 1 {res1} Part 2 {res2}")
}
