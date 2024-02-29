use std::{collections::HashSet, fs, ops::RangeBounds};

pub fn solve() {
    let input =
        fs::read_to_string("./input/day4_scratchcards.txt").expect("Error in reading text file");

    let mut sum_points = 0;
    for line in input.lines() {
        let count = parse_and_get_count(line);
        sum_points += calculate_points(count);
    }

    println!("Sum of Points: {sum_points}");
}

pub fn solve2() {
    let input =
        fs::read_to_string("./input/day4_scratchcards.txt").expect("Error in reading text file");

    let mut card_count = vec![0; input.lines().count()];
    for (i, line) in input.lines().enumerate() {
        card_count[i] += 1;
        let count = parse_and_get_count(line);
        for j in (i as i32)..(i as i32 + count) {
            card_count[j] += 1;
        }
    }

    let sum = card_count.iter().sum();
    println!("Card Count: {sum}");
}

fn parse_and_get_count(line: &str) -> i32 {
    let mut tokens = line.splitn(3, [':', '|']).skip(1);
    let winning_numbers = tokens
        .next()
        .expect("Error in parsing winning numbers")
        .trim();
    let my_numbers = tokens.next().expect("Error in parsing my numbers").trim();

    let winning_numbers = num_str_to_set(winning_numbers);
    let my_numbers = num_str_to_set(my_numbers);

    winning_numbers.intersection(&my_numbers).count() as i32
}

fn calculate_points(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    let mut ret = 1;
    for _ in 0..n - 1 {
        ret *= 2;
    }
    ret
}

fn num_str_to_set(s: &str) -> HashSet<i32> {
    let mut num_hash = HashSet::new();

    for num_str in s.split_whitespace() {
        let num_str: i32 = num_str.parse().expect("Error in parsing num_str!");
        num_hash.insert(num_str);
    }

    num_hash
}
