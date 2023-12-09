use std::{ops::{AddAssign, SubAssign}, collections::HashMap};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Vec2 {
    x: i64,
    y: i64
}

impl Vec2 {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn inverted(&self) -> Self {
        Self { x: self.x * -1, y: self.y * -1 }
    }
}


impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

fn main() {
    let direction_str = include_str!("../input.txt").trim();

    let mut directions: Vec<Vec2> = Vec::new();
    for d in direction_str.chars() {
        directions.push(
            match d {
                '^' => Vec2::new(0, 1),
                'v' => Vec2::new(0, -1),
                '>' => Vec2::new(1, 0),
                '<' => Vec2::new(-1, 0),
                _ => Vec2::new(0,0),
            }
        )
    }

    let mut houses: HashMap<Vec2, i32> = HashMap::new();
    houses.insert(Vec2::new(0, 0), 1);

    let mut current_pos = Vec2::new(0,0);

    for direction in &directions {
        current_pos += *direction;
        if houses.contains_key(&current_pos) {
            *houses.get_mut(&current_pos).unwrap() += 1;
        } else {
            houses.insert(current_pos.clone(), 1);
        }
    }
    let houses_delivered_to_by_santa = houses.keys().len();
    println!("Task 1: {houses_delivered_to_by_santa}");
    houses.clear();
    houses.insert(Vec2::new(0, 0), 2);

    let mut santa_pos = Vec2::new(0,0);
    let mut robo_pos = Vec2::new(0,0);

    for direction in directions.chunks(2) {
        santa_pos += direction[0];
        robo_pos += direction[1];

        for i in [santa_pos, robo_pos] {
            if houses.contains_key(&i) {
                *houses.get_mut(&i).unwrap() += 1;
            } else {
                houses.insert(i.clone(), 1);
            }
        }
        
    }

    let houses_delivered_to = houses.keys().len();
    println!("Task 2: {houses_delivered_to}")
}
