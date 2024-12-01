use std::{collections::HashMap, iter::zip};

// Get the amount of occurances of a value in an array.
fn occurances<T: std::cmp::PartialEq>(num: &T, list: &Vec<T>) -> i32 {
    let mut occ = 0;


    for i in list {
        if num == i {
            occ += 1;
        }

    }



    return occ 
}


fn main() {
    // Load the input and convert it into a vec of &str
    let input: Vec<&str> = include_str!("../input.txt").split("\n").collect();

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in input {
        let (val1, val2) = match line.split_once(" ") {
            Some(v) => v,
            None => {
                continue;
            }            
        };
        // String to number
        let v1 = i32::from_str_radix(val1.replace(" ", "").as_str(), 10).unwrap();
        let v2 = i32::from_str_radix(val2.replace(" ", "").as_str(), 10).unwrap();

        list1.push(v1);
        list2.push(v2);
    }

    list1.sort();
    list2.sort();

    let mut total_dist = 0 ;

    // Calculate the distances.
    for (n1, n2) in zip(&list1, &list2) {
        let dist = (n1 - n2).abs();
        total_dist += dist;
    }

    println!("Distance is: {}", total_dist);

    // Make a lookup table for performance reasons. This is probably not needed here but with larger
    // datasets it will improve performance by not having to calculate the occurances on repeated
    // values in list1
    let mut lookup: HashMap<i32, i32> = HashMap::new();
    let mut sim_score: i32 = 0;

    // Part 2
    for n1 in list1 {
        if lookup.contains_key(&n1) {
            sim_score += n1 * lookup.get(&n1).unwrap();
        } else {
            let occ = occurances(&n1, &list2);
            sim_score += n1 * occ;
            lookup.insert(n1, occ);
        }
    }

    println!("Simalarity score: {}", sim_score);

}
