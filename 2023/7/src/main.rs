use std::{collections::HashMap, ops::AddAssign};

const NOT_JOKER: [&'static str; 12] = [
    "A",
    "K",
    "Q",
    "T",
    "9",
    "8",
    "7",
    "6",
    "5",
    "4",
    "3",
    "2",
];

struct Card {
    value: u8,
}
impl Card {
    fn from_char(ch: char) -> Self {
        let cn: u8 = match ch {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            'J' => 1,
            _ => panic!()
        };

        Self { value: cn }
    }
}

struct Hand {
    original_input: String,
    hand: Vec<Card>,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get_strength().partial_cmp(&other.get_strength())
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.get_strength() == other.get_strength()
    }
}


fn max<T:PartialOrd>(x: T, y: T) -> T {
    if x > y {
        x
    } else {
        y
    }
}
impl Hand {
    fn from_string(text: &str, original: &str) -> Self {


        let h: Vec<Card> = text.chars().map(|x| Card::from_char(x)).collect();
        

        Self { hand: h, original_input: original.to_string()}
    }

    fn from_string_jokerswap(text: &str, original: &str) -> Self {

        let jokers = count_in_array(&text.chars().collect::<Vec<char>>(), 'J');

        

        let mut highest_hand = Self::from_string(text, original);

        if jokers != 0 {
            for i in NOT_JOKER {
                let temp_text = text.replacen("J", i, 1);
                highest_hand = max(highest_hand, Self::from_string_jokerswap(temp_text.as_str(), original));
                
            }
        };



        return highest_hand;

    }

    fn value_add(&self) -> (u128, usize) {
        let mut cv = 0;
        let mut size = 0;
        for i in &self.hand {
            cv = cv << 4;
            size += 4;
            cv += i.value as u128;
        }

        (cv,size)
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
        let fours  = count_in_array(&vals, 4);
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

        let org = Self::from_string(self.original_input.as_str(), "");
        let (va, shift) = org.value_add();
        
        cv = cv << shift;
        cv = cv | va;

        cv

        
    }
}

fn count_in_array<T: PartialEq>(arr: &Vec<T>, to_count: T) -> u32 {
    let mut contains: u32 = 0;
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

        Self { hand: Hand::from_string_jokerswap(h, h), bet: v.parse::<i32>().unwrap() }
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
