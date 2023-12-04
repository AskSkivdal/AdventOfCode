#[derive(Debug)]
struct ScratchCard {
    card_id: i32,
    winning_numbers: Vec<i32>,
    numbers: Vec<i32>
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

        return Self { card_id, winning_numbers, numbers }
    }
    fn get_points(&self) -> i32 {
        let mut points = 0;
        for num in &self.numbers {
            if self.winning_numbers.contains(num) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }


        points
    }
}


fn main() {
    let lines: Vec<&str> = include_str!("../input.txt").split("\n").collect();
    let mut cards: Vec<ScratchCard> = vec![];

    let mut total_points = 0;
    for l in lines {
        cards.push(ScratchCard::from_line(l));
    }

    for card in &cards {
        total_points += card.get_points();
    }

    println!("{}", total_points)
}
