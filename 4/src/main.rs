use std::collections::HashMap;


struct ScratchCard {
    card_id: i32,
    winning_numbers: Vec<i32>,
    numbers: Vec<i32>,
    matches: i32,
}

impl ScratchCard {
    fn from_line(line: &str) -> Self {
        let (preamble, data) = line.split_once(": ").unwrap();
        let (_, number) = preamble.split_once(" ").unwrap();
        let card_id:i32 = number.trim().parse::<i32>().unwrap();

        let (winners, current) = data.split_once(" | ").unwrap();

        let mut winning_numbers: Vec<i32> = Vec::new();
        for winner in winners.split(" ") {
            if winner.is_empty() { continue; }
            winning_numbers.push(winner.parse::<i32>().unwrap())
        }

        let mut numbers: Vec<i32> = Vec::new();

        for num in current.split(" ") {
            if num.is_empty() { continue; }

            numbers.push(num.parse::<i32>().unwrap())
        }

        let mut matches = 0;
        for num in &numbers {
            if winning_numbers.contains(&num) {
                matches += 1;
            }
        }

        return Self { card_id, winning_numbers, numbers, matches }
    }

}


fn main() {
    let mut card_map: HashMap<i32, ScratchCard> = HashMap::new();
    let lines: Vec<&str> = include_str!("../input.txt").split("\n").collect();

    for l in lines {
        let card = ScratchCard::from_line(l);
        card_map.insert(card.card_id.clone(), card);
    }
    let mut processed: i32 = 0;
    let mut current_cards: Vec<i32> = (1..=214).collect();

    loop {
        if current_cards.is_empty() {
            break;
        }

        let mut new_cards: Vec<i32> = Vec::new();
        for card_id in &current_cards {
            processed += 1;
            let current_card = match card_map.get(&card_id) {
                Some(v) => v,
                None => {
                    println!("No card with id: {}", card_id);
                    continue;
                }
            };
            let _match = current_card.matches;
            let mut current_new_cards: Vec<i32> = (card_id.clone()+1..=(card_id.clone()+_match.clone())).collect();
            new_cards.append(&mut current_new_cards);
        }
        current_cards.clear();

        current_cards.append(&mut new_cards);
    }
    assert_eq!(processed, 13261850);
    println!("{}", processed)

}
