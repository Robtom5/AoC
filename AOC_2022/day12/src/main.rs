use pathfinding::prelude::bfs;
use std::fs;

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

fn part1(contents: &str) -> (String, String) {
    let (map, src, dst, sz) = load_map(&contents);

    let result = bfs(&src, |p| p.successors(&map, sz), |p| *p == dst);

    let unwrapped = result.expect("no path found");
    let unwrapped_len = unwrapped.len();

    let pos_result = bfs(&dst, |p| p.successors_inv(&map, sz), |x| map[x.1][x.0] == 1);
    let min_dis = pos_result.unwrap().len();

    return ((unwrapped_len - 1).to_string(), min_dis.to_string());
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
