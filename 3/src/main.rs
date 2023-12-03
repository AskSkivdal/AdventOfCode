use regex::Regex;


const NUMBERS: [char; 10] = [
    '0',
    '1',
    '2',
    '3',
    '4',
    '5',
    '6',
    '7',
    '8',
    '9'
];

#[derive(Debug)]
struct NumberPos {
    // Start: x, y (0,0) == TopLeft
    start: (usize, usize),
    length: usize
}

#[derive(Debug)]
struct Number {
    value: u32,
    valid: bool
}

fn contains_symbols(text: String) -> bool {
    let re: Regex = Regex::new(r"(\.|[0-9])").unwrap();
    let result = re.replace_all(&text, "").to_string();

    return !result.is_empty()
   
}

fn main() {
    let lines: Vec<String> = include_str!("../input.txt").split("\n").map(|x| x.to_string()).collect();
    let mut num_poses: Vec<NumberPos> = vec![];
    for (idx, line) in (&lines).iter().enumerate() {
        let mut last_was_num = false;
        let mut tmp: NumberPos = NumberPos { start: (0,0), length: 0 };
        for (chidx, ch) in line.chars().enumerate() {
            let is_num = NUMBERS.contains(&ch);
            if last_was_num && is_num {
                tmp.length += 1
            } else if last_was_num && !is_num {
                num_poses.push(tmp);
                tmp = NumberPos {start:(0,0), length: 0};
                last_was_num = false;
            } else if is_num && !last_was_num {
                tmp.start = (chidx, idx);
                tmp.length = 1;
                last_was_num = true;
            }
        }
        if last_was_num {
            num_poses.push(tmp);
            
        }
    }

    let mut numbers: Vec<Number> = vec![];

    let line_length: usize = lines[0].len().try_into().unwrap_or(0);
    for i in (&num_poses).iter() {
        let num = lines[i.start.1][i.start.0..i.start.0+i.length].to_string().parse::<u32>().unwrap_or(0);
        let mut valid = false;

        if i.start.1 != 0 {
            let above = lines[i.start.1-1][(((i.start.0 as i32) - 1).clamp(0, line_length as i32) as usize)..(i.start.0+i.length+1).clamp(0, line_length-1)].to_string();
            println!("{above}");
            if contains_symbols(above) {
                valid = true;
            }
        }

        
        if i.start.0 != 0 {
            let left = lines[i.start.1][i.start.0-1..=i.start.0-1].to_string();
            print!("{left}");
            if contains_symbols(left) {
                valid = true;
            }
        }
        print!("{num}");
        if i.start.0+i.length-1 != lines[0].len() {
            let right = lines[i.start.1][(i.start.0+i.length).clamp(0, line_length-1)..=(i.start.0+i.length).clamp(0, line_length-1)].to_string();
            print!("{right}\n");
            if contains_symbols(right) {
                valid = true;
            }
        }
        if i.start.1 != lines.len()-1 {
            let below = lines[i.start.1+1][(((i.start.0 as i32) - 1).clamp(0, line_length as i32) as usize)..(i.start.0+i.length+1).clamp(0, line_length)].to_string();
            println!("{below}");
            if contains_symbols(below) {
                valid = true;
            }
        }
        println!("^^^^^^ - {valid}\n");
        numbers.push(Number { value: num, valid: valid })
    }

    let mut sum: u32 = 0;
    for i in numbers {
        if i.valid {
            sum += i.value;
        }
    }
    println!("Sum is: {sum}");

    

}
