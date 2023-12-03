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

#[derive(Debug, Clone, Copy)]
struct NumberPos {
    // Start: x, y (0,0) == TopLeft
    start: (usize, usize),
    length: usize
}

#[derive(Debug, Clone, Copy)]
struct Number {
    value: u32,
    valid: bool,
    start: (usize, usize),
    length: usize
}

fn contains_symbols(text: String) -> bool {
    let re: Regex = Regex::new(r"(\.|[0-9])").unwrap();
    let result = re.replace_all(&text, "").to_string();

    return !result.is_empty()
   
}

fn is_within_one(pos1: (usize, usize), pos2: (usize, usize)) -> bool {
    let horizontal: bool = pos2.0 >= pos1.0 - 1 && pos2.0 <= pos1.0 + 1;
    let vertical: bool = pos2.1 >= pos1.1 - 1 && pos2.1 <= pos1.1 + 1;

    vertical && horizontal
}

#[test]
fn test_within_one() {
    // Test one more or less, single axis.
    assert_eq!(is_within_one((32,92), (33,92)), true);
    assert_eq!(is_within_one((32,92), (31,92)), true);
    assert_eq!(is_within_one((32,92), (32,93)), true);
    assert_eq!(is_within_one((32,92), (32,91)), true);
    // Test diagonal
    assert_eq!(is_within_one((22,22), (23,23)), true);
    assert_eq!(is_within_one((22,22), (23,21)), true);
    assert_eq!(is_within_one((22,22), (21,21)), true);
    assert_eq!(is_within_one((22,22), (21,22)), true);

    // Test to far away;
    assert_eq!(is_within_one((32,92), (32,30)), false);
}

fn main() {
    // Get the lines
    let lines: Vec<String> = include_str!("../input.txt").split("\n").map(|x| x.to_string()).collect();
    // Get the position of the numbers.
    let mut num_poses: Vec<NumberPos> = vec![];
    for (idx, line) in (&lines).iter().enumerate() {
        // Get all the number on the current line.
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
            if contains_symbols(above) {
                valid = true;
            }
        }

        
        if i.start.0 != 0 {
            let left = lines[i.start.1][i.start.0-1..=i.start.0-1].to_string();
            if contains_symbols(left) {
                valid = true;
            }
        }
        if i.start.0+i.length-1 != lines[0].len() {
            let right = lines[i.start.1][(i.start.0+i.length).clamp(0, line_length-1)..=(i.start.0+i.length).clamp(0, line_length-1)].to_string();
            if contains_symbols(right) {
                valid = true;
            }
        }
        if i.start.1 != lines.len()-1 {
            let below = lines[i.start.1+1][(((i.start.0 as i32) - 1).clamp(0, line_length as i32) as usize)..(i.start.0+i.length+1).clamp(0, line_length)].to_string();
            if contains_symbols(below) {
                valid = true;
            }
        }
        numbers.push(Number { value: num, start: i.start, length: i.length, valid: valid })
    }

    let mut sum: u32 = 0;
    for i in &numbers {
        if i.valid {
            sum += i.value;
        }
    }
    // Sum is: 533784
    println!("Sum is: {sum}");

    // Get gear ratios.
    let mut gear_locations:Vec<(usize, usize)> = vec![];
    for (idx, line) in lines.iter().enumerate() {
        for i in 0..line.len() {
            let current = line.chars().nth(i).unwrap();
            if current == '*' {
                gear_locations.push((i, idx));
            }
        }   
    }

    let mut gear_numbers: Vec<Vec<Number>> = vec![];
    
    for (gear_x, gear_y) in gear_locations {
        let mut current_possibilities: Vec<Number> = vec![];
        for num in &numbers {


            let mut valid = false;
            for offset in 0..num.length {
                if is_within_one((gear_x,gear_y), (num.start.0+offset, num.start.1)) {
                    valid = true;
                    continue
                };

            }
            if valid {
              current_possibilities.push(num.clone())
            }
        }
        gear_numbers.push(current_possibilities)
    }

    let mut gear_ratio_total: u64 = 0;
    
    for gear in &gear_numbers {
        if gear.len() == 2 {
            gear_ratio_total += (gear[0].value as u64 * gear[1].value as u64) as u64;
        }
    }
    // Gear ratio total: 78826761
    println!("Gear ratio total: {gear_ratio_total}");

}
