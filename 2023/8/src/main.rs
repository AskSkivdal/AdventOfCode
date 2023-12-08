use std::collections::HashMap;

pub fn gcd(mut n: u128, mut m: u128) -> u128 {
    assert!(n != 0 && m != 0);
    while m != 0 {
      if m < n {
        std::mem::swap(&mut m, &mut n);
      }
      m %= n;
    }
    n
  }
  

fn main() {
    // Load input
    let (instruction_string, node_defenitions) =
        include_str!("../input.txt").split_once("\n\n").unwrap();
    // Get directions.
    let instructions: Vec<usize> = instruction_string
        .trim()
        .chars()
        .map(|x| if x == 'L' { 0 } else { 1 })
        .collect();

    let mut hm: HashMap<String, [String; 2]> = HashMap::new();
    // Add the map to a hashmap.
    let mut currents: Vec<String> = Vec::new();
    for i in node_defenitions.trim().split("\n") {
        let (current, dirty_pair) = i.split_once(" = ").unwrap();
        let clean_pair = dirty_pair.replace("(", "").replace(")", "");
        let (l, r) = clean_pair.split_once(", ").unwrap();

        hm.insert(current.to_string(), [l.to_string(), r.to_string()]);
        if current.ends_with("A") {
            currents.push(current.to_string())
        }
    }

    // Store all the cycles.
    let mut cycles: Vec<u128> = Vec::new();

    // Loop over all the starting points
    for c in currents {
        let mut current = c.clone();
        
        let mut steps: u128 = 0;

        // Loop until endpoint.
        for idx in instructions.iter().cloned().cycle() {
            steps += 1;
            current = hm.get(&current).unwrap()[idx].to_string();
            let done = current.ends_with("Z");
            if done {break;}
        }
   
        // Push the length of the cycle to a list.
        cycles.push(steps)
        
    }

    // Set the least common multiple to be the popped version of the element
    let mut lcm = cycles.pop().unwrap();

    // Get the lcm for the entire list of cycles
    for num in cycles {
        lcm = lcm * num / gcd(lcm, num)
    }

    // Display the lcm. Yay 🎉
    println!("Traversed in {} steps", lcm);
}
