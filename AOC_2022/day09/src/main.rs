use core::hash::Hash;
use core::hash::Hasher;
use std::cmp::max;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Eq)]
struct Point {
    x: i16,
    y: i16,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

#[derive(Debug)]
struct Rope {
    head: Point,
    tail: Point,
}

fn create_point() -> Point {
    Point { x: 0, y: 0 }
}

fn create_point_from_coords(x: i16, y: i16) -> Point {
    Point { x: x, y: y }
}

fn create_rope() -> Rope {
    Rope {
        head: create_point(),
        tail: create_point(),
    }
}

trait Tail {
    fn update_tail(&mut self) -> Point;
}

trait Distance<Rhs = Self> {
    fn distance_to(&self, other: &Rhs) -> i16;
}

// #[allow(unused)]
impl Tail for Rope {
    fn update_tail(&mut self) -> Point {
        let Point { x: hx, y: hy } = self.head;

        let (dx, dy): (i16, i16) = match self.tail.distance_to(&self.head) {
            _n if _n > 1 => {
                match &self.tail {
                    Point { x, y } if x == &hx => (*x, y + (hy - y).signum()), // Same row
                    Point { x, y } if y == &hy => (x + (hx - x).signum(), *y), //Same column,
                    p => (p.x + (hx - p.x).signum(), p.y + (hy - p.y).signum()), // Point { x, y } => (*x, *y),
                }
            }
            _ => (self.tail.x, self.tail.y),
        };
        self.tail.x = dx;
        self.tail.y = dy;
        create_point_from_coords(dx, dy)
    }
}

impl Distance for Point {
    fn distance_to(&self, other: &Point) -> i16 {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();

        return max(dx, dy);
    }
}

fn part1(contents: &str) -> String {
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(create_point());
    let mut rope = create_rope();

    for line in contents.lines() {
        let (dir, dist) = match line.split_once(" ") {
            Some(n) => n,
            None => continue,
        };
        let dist_val = match dist.parse::<usize>() {
            Ok(n) => n,
            Err(_) => continue,
        };

        for _i in 0..dist_val {
            match dir {
                "R" => rope.head.x += 1,
                "L" => rope.head.x -= 1,
                "U" => rope.head.y += 1,
                "D" => rope.head.y -= 1,
                _ => panic!("Unknown direction"),
            };
            visited.insert(rope.update_tail());
        }
    }
    #[cfg(debug_assertions)]
    printvisible(&visited, 6, 6);
    visited.len().to_string()
}

#[cfg(debug_assertions)]
fn printvisible(set: &HashSet<Point>, width: usize, height: usize) {
    let blankline = ".".repeat(width) + "\n";
    let mut blankgrid = blankline.repeat(height);
    for pt in set {
        let x_val = pt.x as usize;
        let y_val = pt.y as usize;
        let value = y_val * (width + 1) + x_val;
        blankgrid.replace_range(value..(value + 1), "@");
    }
    println!("{blankgrid}");
}

#[allow(unused)]
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
