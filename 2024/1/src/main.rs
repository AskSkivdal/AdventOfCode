use std::iter::zip;

fn main() {
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
        let v1 = i32::from_str_radix(val1.replace(" ", "").as_str(), 10).unwrap();
        let v2 = i32::from_str_radix(val2.replace(" ", "").as_str(), 10).unwrap();

        list1.push(v1);
        list2.push(v2);
    }

    list1.sort();
    list2.sort();

    let mut total_dist = 0 ;

    for (n1, n2) in zip(list1, list2) {
        let dist = (n1 - n2).abs();
        total_dist += dist;
    } 

    println!("Distance is: {}", total_dist)

}
