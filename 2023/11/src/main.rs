const DOMAIN_EXPANTION_VALUE: i64 = 1_000_000;

struct Galaxy {
    x: i64,
    y: i64
}


fn galaxy_manhattan_distance(g1: &Galaxy, g2: &Galaxy) -> i64 {
    let x_dist: i64 = (g2.x - g1.x).abs();
    let y_dist: i64 = (g2.y - g1.y).abs();
    x_dist + y_dist
}

fn main() {
    // Collect the input as a list of bools. True is galaxy, false is not.
    let input: Vec<Vec<bool>> = include_str!("../input.txt").split("\n").filter(|x| !x.is_empty()).map(|x| x.chars().map(|x| x=='#').collect()).collect();

    let mut vertical_expantions: Vec<i64> = vec![];

    for (y, line) in input.iter().enumerate() {
        if !line.contains(&true) {
            vertical_expantions.push(y.clone() as i64);
        }
    }

    let mut horizontal_expantions: Vec<i64> = vec![];
    let width = input.first().unwrap().len();
    for x in 0..width {
        let mut contains_galaxy = false;
        for l in &input {
            if *l.get(x).unwrap() {
                contains_galaxy = true
            }
        }
        if !contains_galaxy {
            horizontal_expantions.push(x.clone() as i64);
        }
    }
    
    println!("{:?}\n{:?}", vertical_expantions, horizontal_expantions);

    let mut galaxys: Vec<Galaxy> = Vec::new();

    for (y, line) in input.iter().enumerate() {
        for (x, v) in line.iter().enumerate() {
            let v_domains = vertical_expantions.iter().filter(|ry| ry < &&(y as i64)).collect::<Vec<&i64>>().len() as i64;
            let h_domains = horizontal_expantions.iter().filter(|rx| rx < &&(x as i64)).collect::<Vec<&i64>>().len() as i64;
            let vert_offset: i64 = ( v_domains * DOMAIN_EXPANTION_VALUE) - v_domains;
            let hori_offset: i64 = ( h_domains * DOMAIN_EXPANTION_VALUE) - h_domains;


            if *v {
                galaxys.push(Galaxy { x: x as i64 + hori_offset, y: y as i64 + vert_offset })
            }
        }
    }
    let mut sum: i64 = 0;
    while !galaxys.is_empty() {
        let current = galaxys.pop().unwrap();
        for g in galaxys.iter() {
            sum += galaxy_manhattan_distance(&current, g)
        }
        
    }

    println!("Task 2: {sum}");
}
