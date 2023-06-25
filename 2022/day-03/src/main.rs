use itertools::Itertools;

static CONTENTS: &str = include_str!("../input");

fn to_priority(c: char) -> u64 {
    if ('a'..='z').contains(&c) {
        (c as u64 - 'a' as u64) + 1
    } else if ('A'..='Z').contains(&c) {
        (c as u64 - 'A' as u64) + 27
    } else {
        panic!("bad char: {}", c);
    }
}

fn priorities(s: &str) -> u64 {
    s.chars().fold(0u64, |acc, c| acc | (1 << to_priority(c)))
}

fn part1() {
    let sum = CONTENTS
        .lines()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);

            let first_priorities = priorities(first);
            let second_priorities = priorities(second);

            let shared = first_priorities & second_priorities;
            shared.trailing_zeros() as u64
        })
        .fold(0u64, |acc, p| acc + p);

    println!("sum of priorities: {}", sum);
}

fn part2() {
    let sum = CONTENTS
        .lines()
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|mut sacks| {
            let first = priorities(sacks.next().unwrap());
            let second = priorities(sacks.next().unwrap());
            let third = priorities(sacks.next().unwrap());

            (first & second & third).trailing_zeros() as u64
        })
        .fold(0u64, |acc, p| acc + p);

    println!("sum of badges for rucksack triplets: {}", sum);
}

fn main() {
    part1();
    part2();
}
