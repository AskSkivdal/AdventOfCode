#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Tile {
    Ground,
    Start,
    Evaluated(i32),
    Pipe(Pipe),
}

impl Tile {
    fn from_char(txt: char) -> Tile {
        match txt {
            '.' => Tile::Ground,
            'S' => Tile::Start,
            '|' | '-' | 'L' | 'J' | '7' | 'F' => Tile::Pipe(Pipe::from_char(txt)),
            _ => Tile::Ground,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_rel_xy(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

enum Enclosement {
    Open,
    Loop,
    Unknown,
    Closed
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Pipe {
    directions: [Direction; 2],
}

impl Pipe {
    fn from_char(pipe: char) -> Pipe {
        match pipe {
            '|' => Pipe {
                directions: [Direction::Up, Direction::Down],
            },
            '-' => Pipe {
                directions: [Direction::Left, Direction::Right],
            },
            'L' => Pipe {
                directions: [Direction::Up, Direction::Right],
            },
            'J' => Pipe {
                directions: [Direction::Left, Direction::Up],
            },
            '7' => Pipe {
                directions: [Direction::Left, Direction::Down],
            },
            'F' => Pipe {
                directions: [Direction::Down, Direction::Right],
            },
            _ => todo!(),
        }
    }
}

fn main() {
    let mut map: Vec<Vec<Tile>> = Vec::new();

    let lines: Vec<&str> = include_str!("../input.txt")
        .split("\n")
        .filter(|x| !x.is_empty())
        .collect();

    for line in lines {
        let l: Vec<Tile> = line.chars().map(|x| Tile::from_char(x)).collect();
        map.push(l);
    }

    let mut starting_pos: (i32, i32) = (0, 0);
    for (y, row) in map.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if value == &Tile::Start {
                starting_pos = (x as i32 , y as i32);
                break;
            }
        }
    }
    let mut to_process: Vec<(i32, i32)> = Vec::new();
    let mut after: Vec<(i32, i32)> = Vec::new();

    for dir in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        let (rel_x, rel_y) = dir.get_rel_xy();
        let (x, y) = starting_pos.clone();

        match &map[(y + rel_y) as usize][(x + rel_x) as usize] {
            Tile::Pipe(p) => {
                for dir2 in &p.directions {
                    let (drel_x, drel_y) = dir2.get_rel_xy();
                    if ((rel_x + drel_x) == 0) && ((rel_y + drel_y) == 0) {
                        to_process.push((x + rel_x, y + rel_y));
                    }
                }
            }
            _ => continue,
        }

        map[y as usize][x as usize] = Tile::Evaluated(0);
    }
    println!("{:?}", to_process);
    loop {
        for (x, y) in to_process.clone() {
            let current = &map[y as usize][x as usize];
            match current {
                Tile::Ground => continue,
                Tile::Start => continue,
                Tile::Evaluated(_) => continue,
                Tile::Pipe(p) => {
                    let mut values: Vec<i32> = Vec::new();
                    for dir in &p.directions {
                        let (rel_x, rel_y) = dir.get_rel_xy();
                        let t = map
                            .get((y+rel_y) as usize)
                            .unwrap()
                            .get((x+rel_x) as usize)
                            .unwrap();
                        match t {
                            Tile::Evaluated(v) => {
                                values.push(v.clone());
                            }
                            Tile::Pipe(p) => {
                                for dir2 in &p.directions {
                                    let (drel_x, drel_y) = dir2.get_rel_xy();
                                    if ((rel_x + drel_x) == 0) && ((rel_y + drel_y) == 0) {
                                        after.push((x + rel_x, y + rel_y));
                                    }
                                }
                            }
                            _ => continue,
                        }
                    }
                    if values.len() == 0 {
                        continue;
                    }
                    let tile_value = values.iter().min().unwrap() + 1;
                    map[y as usize][x as usize] = Tile::Evaluated(tile_value);
                }
            }
        }
        to_process.clear();
        to_process.append(&mut after);
        if to_process.is_empty() {
            println!("Empty");
            break;
        }
    }

    for row in &map {
        for val in row {
            let v = match val {
                Tile::Evaluated(v) => format!("#"),
                _ => " ".to_string()
            };

            print!("{v}");
        }
        println!();
    }
    

    let m = map
        .iter()
        .map(|x| {
            x.iter()
                .map(|x| match x {
                    Tile::Evaluated(v) => v,
                    _ => &0,
                })
                .max()
                .unwrap_or(&0)
        })
        .max()
        .unwrap();

    println!("Deepest within maze: {m}");


}
