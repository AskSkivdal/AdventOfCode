use std::collections::HashMap;

fn main() {
    let (instruction_string, node_defenitions) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let instructions: Vec<usize> = instruction_string.trim().chars().map(|x| if x == 'L' {0} else {1}).collect();


    let mut hm: HashMap<String, [String; 2]> = HashMap::new();

    for i in node_defenitions.trim().split("\n") {
        let (current, dirty_pair) = i.split_once(" = ").unwrap();
        let clean_pair = dirty_pair.replace("(", "").replace(")", "");
        let (l,r) = clean_pair.split_once(", ").unwrap();

        hm.insert(current.to_string(), [l.to_string(),r.to_string()]);
    };

    let mut current: String = "AAA".to_string();
    let mut steps = 0;
    for idx in instructions.iter().cloned().cycle() {
        if current == "ZZZ".to_string() {
            break;
        }
        current = hm.get(&current).unwrap()[idx].to_string();

        steps += 1;
    }

    println!("Traversed in {} steps", steps);
}
