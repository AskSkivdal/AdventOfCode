fn main() {
    let direction: Vec<i32> = include_str!("../input.txt").chars().map(|x| match x {'(' => 1, ')'=> -1, _ => 0}).collect();
    // Part 1
    let mut current_floor = 0;
    let mut first_below_0 = 0;
    for (i, dir) in direction.iter().enumerate() {
        current_floor += dir;
        if current_floor < 0 {
            first_below_0 = i+1;
            break;
        }
    }

    let floor: i32 = direction.iter().sum();
    println!("Floor {floor}");
    println!("First in basement: {first_below_0}");
}
