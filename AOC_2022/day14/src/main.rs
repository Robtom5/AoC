use std::cmp::max;
use std::cmp::min;
use std::fs;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

const SAND_START: Pos = Pos(500, 0);

struct Rock {
    pts: Vec<Pos>,
}

impl Rock {
    fn new() -> Rock {
        Rock { pts: vec![] }
    }

    fn add(&mut self, new_pos: Pos) {
        match self.pts.last() {
            Some(pt) => {
                let &Pos(x0, y0) = pt;
                let Pos(x1, y1) = new_pos;
                match (x1, y1) {
                    (x, _) if x1 == x0 => {
                        for y in range_between(y0, y1) {
                            self.pts.push(Pos(x, y));
                        }
                    }
                    (_, y) if y1 == y0 => {
                        for x in range_between(x0, x1) {
                            self.pts.push(Pos(x, y));
                        }
                    }
                    (x, y) => panic!("Must match either x or y. {x} {y} {x0} {y0}"),
                }
            }
            None => self.pts.push(new_pos),
        }
    }

    fn range(&self) -> (usize, usize, usize) {
        let x_min = self.pts.iter().min_by_key(|x| x.0).unwrap().0;
        let x_max = self.pts.iter().max_by_key(|x| x.0).unwrap().0;
        let y_max = self.pts.iter().max_by_key(|x| x.1).unwrap().1;

        (x_min, x_max, y_max)
    }
}

fn range_between(pt1: usize, pt2: usize) -> Vec<usize> {
    match pt2 > pt1 {
        true => (pt1..=pt2).collect(),
        false => (pt2..=pt1).rev().collect(),
    }
}

fn load_map(contents: &str) -> (Vec<bool>, (usize, usize), usize) {
    let mut x_min = SAND_START.0;
    let mut x_max = 0;
    let mut y_max = 0;
    let mut rocks: Vec<Rock> = Vec::new();
    for line in contents.lines() {
        let mut line_rock = Rock::new();
        let corners = line.split(" -> ").map(|point| {
            let (x, y) = point.split_once(",").unwrap();
            Pos(x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        });

        for point in corners {
            line_rock.add(point);
        }

        let (rx_min, rx_max, ry_max) = line_rock.range();
        x_min = min(x_min, rx_min);
        x_max = max(x_max, rx_max);
        y_max = max(y_max, ry_max);
        rocks.push(line_rock);
    }

    let width = (x_max + 1) - x_min;
    let offset = x_min;
    let height = y_max + 1;
    let sand_start_x = SAND_START.0 - offset;
    let mut map: Vec<bool> = vec![false; width * height];

    for rock in rocks {
        for pt in rock.pts {
            let &Pos(mut x, y) = &pt;
            x -= offset;
            let index = (y * width) + x;
            map[index] = true;
        }
    }

    #[cfg(debug_assertions)]
    print_map(&map, width, height);

    return (map, (width, height), sand_start_x);
}

fn load_map_2(contents: &str) -> (Vec<bool>, (usize, usize), usize) {
    let mut x_min = SAND_START.0;
    let mut x_max = 0;
    let mut y_max = 0;
    let mut rocks: Vec<Rock> = Vec::new();
    for line in contents.lines() {
        let mut line_rock = Rock::new();
        let corners = line.split(" -> ").map(|point| {
            let (x, y) = point.split_once(",").unwrap();
            Pos(x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        });

        for point in corners {
            line_rock.add(point);
        }

        let (rx_min, rx_max, ry_max) = line_rock.range();
        x_min = min(x_min, rx_min);
        x_max = max(x_max, rx_max);
        y_max = max(y_max, ry_max);
        rocks.push(line_rock);
    }

    let offset = 0;
    let width = x_max + (x_min - offset);
    let height = y_max + 3;
    let sand_start_x = SAND_START.0 - offset;
    let mut map: Vec<bool> = vec![false; width * height];

    for rock in rocks {
        for pt in rock.pts {
            let &Pos(mut x, y) = &pt;
            x -= offset;
            let index = (y * width) + x;
            map[index] = true;
        }
    }

    for i in 0..width {
        map[(height - 1) * width + i] = true
    }

    return (map, (width, height), sand_start_x);
}

#[allow(dead_code)]
fn print_map(map: &Vec<bool>, width: usize, height: usize) {
    println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    for row in 0..height {
        for n in &map[(width * row)..(width * (row + 1))] {
            match n {
                true => print!("x"),
                false => print!("."),
            }
        }
        println!("");
    }
}

fn assess_map(map: &mut Vec<bool>, width: usize, height: usize, sand_start: usize) -> String {
    let mut sand_grains = 0;

    let mut grain_x = sand_start;
    let mut grain_y = 0;
    loop {
        match map[sand_start] {
            true => break, // Sand at the start
            false => {}
        }
        let grain_index = (grain_y * width) + grain_x;
        // println!("{esc}[1;1H", esc = 27 as char);
        match grain_y {
            n if n == (height - 1) => break,
            _ => {}
        }
        match map[grain_index + width] {
            false => {
                grain_y += 1;
            }
            true => match grain_x {
                0 => break,
                _ => {
                    let _ff = 2;
                    match map[grain_index + width - 1] {
                        false => {
                            grain_y += 1;
                            grain_x -= 1;
                        }
                        true => match grain_x {
                            n if n == width => break,
                            _ => match map[grain_index + width + 1] {
                                false => {
                                    grain_y += 1;
                                    grain_x += 1;
                                }
                                true => {
                                    map[grain_index] = true;
                                    sand_grains += 1;
                                    grain_y = 0;
                                    grain_x = sand_start;
                                }
                            },
                        },
                    }
                }
            },
        }
        // print_map(&map, width, height);
    }

    return sand_grains.to_string();
}

fn part1(contents: &str) -> String {
    let (mut map, (width, height), sand_start) = load_map(&contents);
    assess_map(&mut map, width, height, sand_start)
}

fn part2(contents: &str) -> String {
    let (mut map, (width, height), sand_start) = load_map_2(&contents);
    assess_map(&mut map, width, height, sand_start)
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

    // Sand coming in at 500,0. Sand falls down, then down left, then down right. New sand appears when sand can't move.
    // How many sand falls in before it starts falling into abyss?
    // Pseudo code/ rough thoughts:
    // Once one grain falls in the abyss all future ones will do so
    // Abyss is just one y coord further than the lowest point (or highest y index) in our shape.
    // We can start by making a sand granule fall straight to lowest point below it
    // We can just make an array that indicates if anything is in a cell instead of a bunch of objects
    // Don't forget to discount the last granule that falls off.

    // while y < abyss
    // next_pos = [(0,+1), (-1, +1), (1, 1)]
    // for p in pos
    // loop
    //    (x,y) = map.get(x+pos.0, y+pos.0){
    //     some(1) => continue,
    //     some(0) => move here
    //     None => in the abyss
    // }
}
