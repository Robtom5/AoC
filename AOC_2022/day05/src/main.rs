use std::fs;
use regex::Regex;

const ASCII_UPPER:[char; 26]= [
    'A', 'B', 'C', 'D', 'E',
    'F', 'G', 'H', 'I', 'J',
    'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y',
    'Z',
]; 



fn init_stacks(contents: &str) -> Vec<Vec<u16>>{
    let mut lines = contents
        .lines()
        .peekable();

    let first_line = lines
        .peek()
        .cloned()
        .unwrap();

    let number_of_stacks = (first_line.len() + 1) / 4;
    let mut layout: Vec<Vec<u16>> = Vec::with_capacity(number_of_stacks as usize);
    for _i in 0..number_of_stacks{
        layout.push(Vec::new());
    }

    for line in lines{
        let line_len = line.len();
        for i in (0..line_len).step_by(4){
            let diff = match line_len - i {
                x if x < 4 => x,
                _ => 4, 
            };
            // let chars = match line_len - i {
            //     n if n < 4 => &mut line[i..i+n].chars(),
            // }
            let chars = &mut line[i..i+diff].chars();
            let box_char = chars.nth(1).unwrap();
            let stack_id: usize = i /4;
            match ASCII_UPPER.iter().any(|&x| x == box_char){  
                true => layout[stack_id].insert(0, char_to_int(box_char)),
                false => continue,
            };
        }
    }

    return layout;
}

fn parse_instruction(line: &str) -> Result<(u16, usize, usize), &str>{
    let re = Regex::new(r"^move (?P<volume>\d+) from (?P<src>\d+) to (?P<dst>\d+)$").unwrap();
    return match re.is_match(line){
        true => Ok(split_cap(re.captures_iter(line))),
        false => Err("No valid instructions"),
    }
}   

fn split_cap(mut captures: regex::CaptureMatches) -> (u16, usize, usize) {
    let cap = captures.next().unwrap();
    let volume = &cap["volume"].parse::<u16>().unwrap();
    let src = &cap["src"].parse::<usize>().unwrap() - 1;
    let dst = &cap["dst"].parse::<usize>().unwrap() - 1;
    return (*volume, src, dst)
}

fn apply_instructions(stacks: &mut Vec<Vec<u16>>, contents: &str){
    let lines = contents.lines();

    for line in lines {
        let (volume, src, dst) = match parse_instruction(line){
            Ok(n) => n,
            Err(_e) => continue,
        };

        for _i in 0..volume {
            let box_to_move = stacks[src].pop().unwrap();
            stacks[dst].push(box_to_move);
        }
    }
}

fn char_to_int(c:char) -> u16{
    return ASCII_UPPER
        .iter()
        .position(|&x| x == c)
        .expect("Can't find box character")
        .try_into()
        .unwrap()
}

fn int_to_char(i:u16) -> char {
    return ASCII_UPPER[i as usize];
}


fn main() {
    let fp = if cfg!(debug_assertions) {
        "data/example"
    } else {
        "data/input"
    }.to_string();

    let contents = fs::read_to_string(fp)
        .expect("Should be able to read file");

    let mut stacks = init_stacks(&contents);

    apply_instructions(&mut stacks, &contents);

    let mut final_str = "".to_owned();

    for stack in stacks {
        let mut ch: char = '!';
        for c in stack{
            ch = int_to_char(c);
            print!("{ch} ")
        }
        final_str.push(ch);

        println!("")
    }   

    println!("Final Str {final_str}")
}