use std::thread::available_parallelism;

fn main() {
    let lines: Vec<&str> = include_str!("../input.txt").split("\n").filter(|x| !x.is_empty()).collect();

    let TIME: i64 = lines[0].split_once(": ").unwrap().1.trim().replace(" ", "").parse::<i64>().unwrap();
    let DISTANCE: i64 = lines[1].split_once(": ").unwrap().1.trim().replace(" ", "").parse::<i64>().unwrap();



    
        let mut wins: Vec<i64> = Vec::new();
        let mut losses: Vec<i64> = Vec::new();
        
        for hold_time in 0..TIME {
            let time_left = TIME-hold_time;
            let dist = time_left*hold_time;
            if dist > DISTANCE {
                wins.push(hold_time);
            } else if dist < DISTANCE {
                losses.push(hold_time)
            }
        }



    
    println!("{}", wins.len());

}
