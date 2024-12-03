use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;

fn main() {
    let data = fs::read_to_string("../../inputs/input1.txt").expect("File not read");
    // println!("{data}");
    let lines = data.lines().map(|line| line.trim());

    let mut list1 = BinaryHeap::new();
    let mut list2 = BinaryHeap::new();

    let mut ans = 0;

    for line in lines {
        let mut nums = line.split_whitespace();
        if let (Some(num1), Some(num2)) = (nums.next(), nums.next()) {
            let num1: i32 = num1.parse().expect("Invalid number");
            let num2: i32 = num2.parse().expect("Invalid number");

            list1.push(Reverse(num1));
            list2.push(Reverse(num2));
        }
    }

    while let (Some(Reverse(num1)), Some(Reverse(num2))) = (list1.pop(), list2.pop()) {
        ans += (num1 - num2).abs();
    }

    println!("Result: {ans}");
}
