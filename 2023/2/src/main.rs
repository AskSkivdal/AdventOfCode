

fn max(x: u64, y:u64) -> u64 {
    if x > y {
        x
    } else {
        y
    }
}

const LIMIT_R: u64 = 12;
const LIMIT_G: u64 = 13;
const LIMIT_B: u64 = 14;

#[derive(Debug)]
pub struct Bag {
    id: u64,
    red: u64,
    green: u64,
    blue: u64,
}

impl Bag {
    pub fn from_record(rec: String) -> Result<Self, ()> {
        let mut id = 0;
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let (mut game, data) = match rec.split_once(": ") {
            Some(v) => v,
            None => return Err(())
        };
        game = match game.strip_prefix("Game ") {
            Some(v) => v,
            None => return Err(())
        };

        id = match game.parse::<u64>() {
            Ok(v) => v,
            Err(_) => return Err(())
        };


        let games: Vec<String> = data.split("; ").map(|x| {x.to_string()}).collect();
        for g in games {
            let pulls: Vec<String> = g.split(", ").map(|x| {x.to_string()}).collect();
            for pull in pulls {
                let (value, key) = match pull.split_once(" ") {
                    Some(v) => v,
                    None => return Err(())
                };

                let value_u = match value.parse::<u64>() {
                    Ok(v) => v,
                    Err(_) => return Err(())
                };

                match key {
                    "red" => {
                        red = max(red, value_u)
                    },
                    "green" => {
                        green = max(green, value_u)

                    },
                    "blue" => {
                        blue = max(blue, value_u)

                    },
                    _ => {
                        return Err(())
                    }
                }
                
            }
        }


        Ok(Bag { id, red, green, blue })
    }

    pub fn validate(self: &Self) -> bool {
        if (self.red > LIMIT_R) | (self.blue > LIMIT_B) | (self.green > LIMIT_G) {
            return false
        }
        
        true
    }

    pub fn power(self: &Self) -> u64 {
        self.red * self.green * self.blue
    }
}


fn main() {
    let lines: Vec<String> = include_str!("../input.txt").split("\n").map(|x| {x.to_string()}).collect();
    let mut bags: Vec<Bag> = Vec::new();

    for i in lines {
        match Bag::from_record(i) {
            Ok(v) => bags.push(v),
            Err(_) => continue
        }

    }
    let mut sum: u64 = 0;
    for i in bags {
        sum += i.power()
    }

    println!("{sum}")
}
