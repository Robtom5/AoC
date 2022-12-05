use std::fs;

// const priorities:Vec<char>= "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(); 
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

    let stacks = init_stacks(&contents);

    for stack in stacks {
        for c in stack{
            let ch = int_to_char(c);
            print!("{ch} ")
        }
        println!("")
    }    
}