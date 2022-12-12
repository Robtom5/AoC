use core::hash::Hash;
use core::hash::Hasher;
use itertools::Itertools;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashSet;

pub fn day06(contents: &str) -> (String, String) {
    fn incr_buff_pos(index: usize, buff_sz: usize) -> usize {
        return match index {
            n if n >= buff_sz - 1 => 0,
            _ => index + 1,
        };
    }

    trait Uniq {
        fn is_unique(&self) -> bool;
    }

    impl Uniq for Vec<char> {
        fn is_unique(&self) -> bool {
            return self.len() == self.into_iter().unique().count();
        }
    }

    fn find_start(contents: &str, buff_sz: usize) -> Result<usize, &str> {
        let mut rb: Vec<char> = vec!['!'; buff_sz];

        let mut buff_index: usize = 0;
        let mut char_index: usize = 0;

        for _char in contents.chars() {
            rb[buff_index] = _char;
            char_index += 1;
            buff_index = incr_buff_pos(buff_index, buff_sz);

            if (char_index > buff_sz) && (rb.is_unique()) {
                break;
            }
        }
        return Ok(char_index);
    }

    let packet_index = match find_start(&contents, 4) {
        Ok(n) => n,
        Err(i) => panic!("{i}"),
    };

    let message_index = match find_start(&contents, 14) {
        Ok(n) => n,
        Err(i) => panic!("{i}"),
    };
    return (packet_index.to_string(), message_index.to_string());
}

pub fn day07(contents: &str) -> (String, String) {
    fn relevant_sz((name, sz): &(String, u64), start: &str) -> u64 {
        match name.starts_with(start) {
            true => return *sz,
            false => return 0,
        }
    }
    let mut path: String = "".to_owned();
    let mut files: Vec<(String, u64)> = Vec::new();
    let mut dirs: HashSet<String> = HashSet::new();

    for line in contents.lines() {
        let words: Vec<&str> = line.split(" ").collect();
        match words[0] {
            "$" => match words[1] {
                "cd" => match words[2] {
                    ".." => {
                        let (new_path, _) = path.rsplit_once('/').unwrap();
                        path = new_path.to_string();
                    }
                    "/" => {
                        path = "".to_string();
                        dirs.insert(path.clone());
                    }
                    w => {
                        let addition = format!("{root}/{dir}", root = path, dir = w);
                        dirs.insert(addition.clone());
                        path = addition;
                    }
                },
                _ => continue,
            },
            "dir" => continue,
            sz => {
                let name = format!("{path}/{file}", path = path, file = words[1]);
                files.push((name, sz.parse::<u64>().unwrap()));
            }
        }
    }

    const MAX_SZ: u64 = 100000;
    const TOTAL_DISK_SZ: u64 = 70000000;
    const MIN_SZ: u64 = 30000000;

    let already_available: u64 =
        TOTAL_DISK_SZ - files.iter().map(|x| relevant_sz(x, "/")).sum::<u64>();
    let size_to_delete = MIN_SZ - already_available;

    assert!(MAX_SZ < size_to_delete); // We haven't accounted for this in the match statement so ensure if is is true
    let mut running_tot = 0;
    let mut best_min = u64::MAX;

    for d in dirs {
        let tot_siz: u64 = files.iter().map(|x| relevant_sz(x, &d)).sum();

        match tot_siz {
            n if n < MAX_SZ => running_tot += n,
            n if n > size_to_delete => best_min = min(best_min, n),
            _ => {}
        }
    }

    return (running_tot.to_string(), best_min.to_string());
}

pub fn day08(contents: &str) -> (String, String) {
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

    let mut best_score = 0;
    for row in 0..height {
        for col in 0..width {
            match score_tree(&contents, (row, col), (height, width)) {
                n if n > best_score => best_score = n,
                _ => {}
            }
        }
    }

    (visible.len().to_string(), best_score.to_string())
}

pub fn day09(contents: &str) -> (String, String) {
    #[derive(Eq)]
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

    trait RopePhysics {
        fn update_tail(&mut self) -> Point;
        fn update_head(&mut self, pt: &Point);
    }

    trait Distance<Rhs = Self> {
        fn distance_to(&self, other: &Rhs) -> i16;
    }
    impl RopePhysics for Rope {
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

        fn update_head(&mut self, pt: &Point) {
            self.head.x = pt.x;
            self.head.y = pt.y;
        }
    }

    impl Distance for Point {
        fn distance_to(&self, other: &Point) -> i16 {
            let dx = (self.x - other.x).abs();
            let dy = (self.y - other.y).abs();

            return max(dx, dy);
        }
    }

    fn rope_behaviour(contents: &str, rope_len: usize) -> String {
        let mut visited: HashSet<Point> = HashSet::new();

        visited.insert(create_point());

        let mut ropes: Vec<Rope> = Vec::with_capacity(rope_len);
        for _i in 0..rope_len {
            ropes.push(create_rope());
        }

        for line in contents.lines() {
            let (dir, dist) = match line.split_once(" ") {
                Some(n) => n,
                None => continue,
            };
            let dist_val = match dist.parse::<usize>() {
                Ok(n) => n,
                Err(_) => continue,
            };

            // let mut rope = &ropes[0];
            // let mut tail;

            for _i in 0..dist_val {
                let mut rope_iter = ropes.iter_mut();
                let mut rope = rope_iter.next().unwrap();
                match dir {
                    "R" => rope.head.x += 1,
                    "L" => rope.head.x -= 1,
                    "U" => rope.head.y += 1,
                    "D" => rope.head.y -= 1,
                    _ => panic!("Unknown direction"),
                };

                let mut last_tail = rope.update_tail();
                for r in rope_iter {
                    r.update_head(&last_tail);
                    last_tail = r.update_tail();
                }
                visited.insert(last_tail);
            }
        }
        visited.len().to_string()
    }

    (rope_behaviour(&contents, 1), rope_behaviour(&contents, 9))
}

pub fn day10(contents: &str) -> (String, String) {
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
            _n if _n <= 1 => print!("██"),
            _ => print!("  "),
        }
        match word.parse::<i32>() {
            Ok(n) => register += n,
            Err(_) => {}
        }
        if (cycle % 40) == 0 {
            println!("");
        }
    }

    return (strength_sum.to_string(), "⬆".to_string());
}
