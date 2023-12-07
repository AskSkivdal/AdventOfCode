use std::{collections::HashMap, ops::AddAssign};

struct Card {
    value: u8,
}
impl Card {
    fn from_char(ch: char) -> Self {
        let cn: u8 = match ch {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'J' => 10,
            'T' => 9,
            '9' => 8,
            '8' => 7,
            '7' => 6,
            '6' => 5,
            '5' => 4,
            '4' => 3,
            '3' => 2,
            '2' => 1,
            _ => panic!()
        };

        Self { value: cn }
    }
}

struct Hand {
    hand: Vec<Card>,
}


fn max(x: u128, y: u128) -> u128 {
    if x > y {
        x
    } else {
        y
    }
}
impl Hand {
    fn from_string(text: &str) -> Self {


        let h: Vec<Card> = text.chars().map(|x| Card::from_char(x)).collect();
        

        Self { hand: h }
    }

    fn get_strength(&self) -> u128 {
        let mut hhashmap: HashMap<u8, u8> = HashMap::new();

        for c in &self.hand {
            
            match hhashmap.get_mut(&c.value) {
                Some(v) => {
                    v.add_assign(1);
                },
                None => {
                    hhashmap.insert((&c.value).clone(), 1);
                }
            };
        }

        let vals: Vec<u8> = hhashmap.values().map(|x| x.clone()).collect();
        let singles = count_in_array(&vals, 1);
        let twos = count_in_array(&vals, 2);
        let threes = count_in_array(&vals, 3);
        let fours: u8 = count_in_array(&vals, 4);
        let fives = count_in_array(&vals, 5);
        let mut cv: u128 = 0;
        // Five of a kind
        if fives == 1 {
            cv = max(cv, 7);
        }
        // Four of a kind
        if fours == 1 {
            cv = max(cv, 6)
        }
        
        // Full house
        if threes == 1 && twos == 1 {
            cv = max(cv, 5)
        }

        // Three of a kind
        if threes == 1 {
            cv = max(cv, 4)
        }
        
        // Two pair
        if twos == 2 {
            cv = max(cv, 3)
        }

        // One pair
        if twos == 1 {
            cv =  max(cv, 2)
        }

        // High card
        if singles == 5 {
            cv =  max(cv, 1);
        }



        for i in &self.hand {
            cv = cv << 4;
            cv += i.value as u128;
        }

        cv

        
    }
}

fn count_in_array(arr: &Vec<u8>, to_count: u8) -> u8 {
    let mut contains: u8 = 0;
    for i in arr {
        if i == &to_count {
            contains += 1;
        }
    }

    contains
}

struct Pull {
    hand: Hand,
    bet: i32,
}

impl Pull {
    fn from_tuple(tup: (&str, &str)) -> Self {
        let (h, v) = tup;

        Self { hand: Hand::from_string(h), bet: v.parse::<i32>().unwrap() }
    }   
}


fn main() {
    let tuples: Vec<(&str, &str)> = include_str!("../input.txt").split("\n").filter(|x| !x.is_empty()).map(|x| x.split_once(" ").unwrap()).collect();
    let mut pulls: Vec<Pull> = Vec::new();
    for tup in tuples {
        pulls.push(Pull::from_tuple(tup));
    }

    pulls.sort_by(|x,y| x.hand.get_strength().cmp(&y.hand.get_strength()));

    let mut sum: u128 = 0;

    for (i, pull) in pulls.iter().enumerate() {
        sum += ((i+1) as i32*pull.bet as i32) as u128
    }

    println!("{sum}")

}
