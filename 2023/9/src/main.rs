/* 
    Adds the next number to a sequence of numbers by finding the increments 
    for the sequence and finding the increments for that after until it dosnt
    increment. Then it has reached a state where it can evaluate the next in
    the sequence by using the generated sequences. 
*/
fn add_next(sequence: &Vec<i32>) -> Vec<i32> {
    // Get a local copy of the sequence.
    let seq = sequence.clone();
    let mut all_seqs: Vec<Vec<i32>> = vec![seq];

    // Loop until it has gotten all the data it needs to extrapolate the next
    // number in the array
    'mainloop: loop {
        let mut current_increments: Vec<i32> = Vec::new();
        /*
            Use a window to get all a slice of the vec that moves across the vec.
            Then subtract the first element in the window with the second to get
            the difference between them, then add it to a vector
        */
        for i in all_seqs.last().unwrap().windows(2) {
            current_increments.push(i[1] - i[0])
        }

        /*
            If there is only zeros in the array it wont increment any more. When
            this happenes all the required data is gotten so it breaks out from 
            the main loop.
         */
        let mut all_zero = true;
        for i in &current_increments {
            if i != &0 {
                all_zero = false;
                break;
            }
        }
        // Add the current increments to the list of sequences.
        all_seqs.push(current_increments);
        if all_zero {break 'mainloop;}
    }
    // Reverse the sequences so it works from top down not bottom up.
    all_seqs.reverse();

    for i in 0..all_seqs.len() {
        // If the index is 0 then push a zero to the current sequence and jump to the next sequence
        if i == 0 {
            all_seqs[i].push(0);
            continue;
        }
        // Get the current increment
        let increment = all_seqs[i-1].last().unwrap().clone();
        // Get the last number in the current sequence.
        let current_last = all_seqs[i].last().unwrap().clone();
        // Increment the last number with the current increment and push it to the current sequence.
        all_seqs[i].push(current_last+increment);
    }
    // Return the last vec in all the sequences. Aka the input element plus the last value.
    return all_seqs.last().unwrap().clone();
}

/*
    Adds the last element by reversing the input sequence and passing it to get_next()
*/
fn add_last(sequence: &Vec<i32>) -> Vec<i32> {
    let mut seq = sequence.clone();
    seq.reverse();
    let mut last = add_next(&seq);
    last.reverse();
    last
}


fn main() {
    // Read all the lines into a vec. Input file MUST use LF line endings not CRLF.
    let lines: Vec<&str> = include_str!("../input.txt").split("\n").filter(|x| !x.is_empty()).collect();

    let mut sequences_to_process: Vec<Vec<i32>> = Vec::new();


    for i in lines {
        // Push a vec of numbers from the read in lines filtered for empty enements. Numbers are split by spaces.
        sequences_to_process.push(i.split(" ").filter(|x| !x.is_empty()).map(|x| x.parse::<i32>().unwrap()).collect())
    }

    let mut nexts: Vec<i32> = Vec::new();
    let mut lasts: Vec<i32> = Vec::new();

    // Get all the next and last numbers.
    for seq in sequences_to_process {
        // Get the sequence with the new value added on.
        let new_sequence = add_next(&seq);
        // Get the last number aka the one that got added from the new_sequence.
        let next = new_sequence.last().unwrap();
        // Push the number to the next values.
        nexts.push(next.clone());

        // Same as above but works on the first element in the vector.
        let new_sequence = add_last(&seq);
        let last = new_sequence.first().unwrap();
        lasts.push(last.clone());
    }
    // Display the sums of the next numbers and the previous numbers.
    let total_forward: i32 = nexts.iter().sum();
    let total_backward: i32 = lasts.iter().sum();
    println!("Part 1: {:?}", total_forward);
    println!("Part 2: {:?}", total_backward);
}
