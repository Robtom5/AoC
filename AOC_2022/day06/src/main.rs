use std::fs;
use itertools::Itertools;

fn incr_buff_pos(index: usize, buff_sz: usize) -> usize {
    return match index {
        n if n >= buff_sz - 1 => 0,
        _ => index + 1,
    };
}

fn decr_buff_pos(index: usize, buff_sz: usize) -> usize {
    return match index {
        n if n <= 0 => buff_sz - 1,
        _ => index - 1,
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

fn find_start(contents: &str, buff_sz: usize) -> Result<(usize, String), &str> {
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
    let mut buffer_contents = "".to_owned();

    for _ in 0..buff_sz {
        buffer_contents.push(rb[buff_index]);
        buff_index = decr_buff_pos(buff_index, buff_sz);
    }

    let message = buffer_contents.to_owned();
    return Ok((char_index, message));
    // return match char_index {
    //     n if n < contents.len() => Ok((char_index, &message)),
    //     _ => Err("No match found"),
    // };
}

fn main() {
    let fp = if cfg!(debug_assertions) {
        "data/example"
    } else {
        "data/input"
    }
    .to_string();

    let contents = fs::read_to_string(fp).expect("Should be able to read file");

    let (index, packet) = match find_start(&contents, 4) {
        Ok(n) => n,
        Err(i) => panic!("{i}"),
    };
    println!("Packet {packet} starts at {index}");

    let (index, message) = match find_start(&contents, 14) {
        Ok(n) => n,
        Err(i) => panic!("{i}"),
    };
    println!("Message {message} starts at {index}")
}
