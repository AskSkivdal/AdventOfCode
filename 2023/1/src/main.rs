const REP_MAP: [(&'static str, &'static str); 9] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

fn replace_first_with_number(text: String) -> String {
    let mut new_str: String = String::new();
    let mut idx: usize = 0;
    let mut last_idx = idx;
    let target = text.len();
    loop {
        for (f,t) in REP_MAP {
            if f.len()+idx > target {
                continue;
            }
            if text[idx..idx+f.len()].contains(f) {
                for i in t.chars().collect::<Vec<char>>().iter() {
                    new_str.push(*i);
                }
                idx += t.len();
            }
        }

        if idx == target {
            break
        }

        if idx == last_idx {
            new_str.push(text.chars().collect::<Vec<char>>()[idx]);
            idx += 1;
        }

        last_idx = idx;
        
    }

    return new_str.to_string();
}

fn main() {
    let mut input: Vec<&str> = include_str!("../input.txt").split("\n").collect();

    let mut nums: Vec<i32> = Vec::new();

    for i in input.iter_mut() {
        let inp = replace_first_with_number(i.to_string());
        println!("{inp}");
        let mut chars: Vec<char> = Vec::new();
        for c in inp.chars() {
            match c.to_string().parse::<i32>() {
                Ok(_) => chars.push(c),
                Err(_) => continue
            }
        }
        let f = match chars.first() {
            Some(v) => v,
            None => continue
        };
        let l = match chars.last() {
            Some(v) => v,
            None => continue
        };

        let fl = format!("{f}{l}");
        match fl.parse::<i32>() {
            Ok(v) => nums.push(v),
            Err(_) => println!("ERIUNBADSFIU")
        } 
    }

    let mut sum: i32 = 0;
    for i in nums {
        sum += i;
    }

    println!("{sum}")
}
