use std::thread::available_parallelism;

fn main() {
    let lines: Vec<&str> = include_str!("../input.txt").split("\n").filter(|x| !x.is_empty()).collect();

    let times: Vec<i32> = lines[0].split_once(": ").unwrap().1.trim().split(" ").filter(|x| !x.is_empty()).map(|x| x.parse::<i32>().unwrap()).collect();
    let distances: Vec<i32> = lines[1].split_once(": ").unwrap().1.trim().split(" ").filter(|x| !x.is_empty()).map(|x| x.parse::<i32>().unwrap()).collect();
    println!("{:?}", times);
    println!("{:?}", distances);
    let groupings: Vec<(&i32, &i32)> = times.iter().zip(&distances).collect::<Vec<(&i32, &i32)>>();


    let mut all_games: Vec<(Vec<i32>, Vec<i32>)> = Vec::new();

    for (time, distance) in groupings {
        let mut wins: Vec<i32> = Vec::new();
        let mut losses: Vec<i32> = Vec::new();
        
        for hold_time in 0..*time {
            let time_left = time-hold_time;
            let dist = time_left*hold_time;
            println!("{}, {}", dist, distance);
            if dist > *distance {
                wins.push(hold_time);
            } else if dist < *distance {
                losses.push(hold_time)
            }
        }
        all_games.push((wins, losses));
    }



    let mut ways_to_win = all_games.iter().map(|x| x.0.len()).collect::<Vec<usize>>();
    let mut margin = ways_to_win.pop().unwrap();
    for i in ways_to_win {
        margin *= i
    }

    println!("{}",margin)

}
