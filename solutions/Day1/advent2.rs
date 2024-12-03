use std::collections::HashMap;
use std::fs;

fn main() {
    // Read the input file
    let data = fs::read_to_string("../../inputs/input2.txt").expect("File not read");
    let lines: Vec<&str> = data.lines().map(|line| line.trim()).collect();

    let mut map = HashMap::new();

    for line in &lines {
        let mut nums = line.split_whitespace();
        if let (Some(_num1), Some(num2)) = (nums.next(), nums.next()) {
            let count = map.entry(num2.to_string()).or_insert(0);
            *count += 1;
        }
    }

    // println!("Map: {:?}", map);

    let mut total_similarity_score = 0;

    for line in &lines {
        let mut nums = line.split_whitespace();
        if let (Some(num1), Some(_num2)) = (nums.next(), nums.next()) {
            if let Ok(num1_int) = num1.parse::<i32>() {
                if let Some(&frequency) = map.get(num1) {
                    total_similarity_score += num1_int * frequency;
                }
            }
        }
    }

    println!("Total Similarity Score: {total_similarity_score}");
}
