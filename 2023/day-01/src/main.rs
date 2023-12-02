use itertools;
use regex::Regex;

static NUMS: &[(&'static str, u16); 9] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

struct NumberFinder {
    forward_re: Regex,
    rev_re: Regex,
}

impl NumberFinder {
    fn new() -> Self {
        let mut nums: Vec<String> = NUMS.iter().map(|(s, _)| (*s).into()).collect();

        let fwd_pat = itertools::join(&nums, "|");
        let forward_re = Regex::new(&fwd_pat).expect("forward_re");

        for num in &mut nums {
            *num = num.chars().rev().collect();
        }

        let rev_pat = itertools::join(nums, "|");
        let rev_re = Regex::new(&rev_pat).expect("rev_re");

        Self { forward_re, rev_re }
    }

    fn find_first_number(&self, s: &str, reversed: bool, recognize_spelled: bool) -> u16 {
        let haystack: String = if reversed {
            s.chars().rev().collect()
        } else {
            s.into()
        };

        let mut num_offset = usize::MAX;
        let n = match haystack.find(|c: char| c.is_digit(10)) {
            Some(offset) => {
                num_offset = offset;
                haystack[num_offset..num_offset + 1]
                    .parse::<u16>()
                    .expect("parse digit")
            }
            None => 0,
        };

        'spelling: {
            if recognize_spelled {
                let pat = if reversed {
                    &self.rev_re
                } else {
                    &self.forward_re
                };

                if let Some(range) = pat.find(&haystack) {
                    let spelled_offset = range.start();

                    if spelled_offset > num_offset {
                        break 'spelling;
                    }

                    let offset = if reversed {
                        s.len() - range.end()
                    } else {
                        spelled_offset
                    };

                    let mut found = None;
                    for (spelled, d) in NUMS.iter().copied() {
                        let slice = &s[offset..];
                        if slice.starts_with(spelled) {
                            found = Some(d);
                            break;
                        }
                    }

                    return found.expect("found");
                }
            }
        }

        n
    }
}

#[test]
fn number_finding() {
    let nf = NumberFinder::new();

    assert_eq!(nf.find_first_number("one5two", false, true), 1);
    assert_eq!(nf.find_first_number("zone5two", false, true), 1);
    assert_eq!(nf.find_first_number("one5two", false, true), 1);
    assert_eq!(nf.find_first_number("zone5two", false, true), 1);
    assert_eq!(nf.find_first_number("one5two", false, true), 1);
    assert_eq!(nf.find_first_number("zone5two", false, true), 1);

    assert_eq!(nf.find_first_number("one5two", false, false), 5);
    assert_eq!(nf.find_first_number("zone5two", false, false), 5);
    assert_eq!(nf.find_first_number("one5two", false, false), 5);
    assert_eq!(nf.find_first_number("zone5two", false, false), 5);
    assert_eq!(nf.find_first_number("one5two", false, false), 5);
    assert_eq!(nf.find_first_number("zone5two", false, false), 5);

    assert_eq!(nf.find_first_number("one5two", true, true), 2);
    assert_eq!(nf.find_first_number("zone5", true, true), 5);
    assert_eq!(nf.find_first_number("one5x", true, true), 5);
    assert_eq!(nf.find_first_number("zone5x", true, true), 5);
    assert_eq!(nf.find_first_number("5oney", true, true), 1);
    assert_eq!(nf.find_first_number("z5one", true, true), 1);

    assert_eq!(nf.find_first_number("one5two", true, false), 5);
    assert_eq!(nf.find_first_number("zone5", true, false), 5);
    assert_eq!(nf.find_first_number("one5x", true, false), 5);
    assert_eq!(nf.find_first_number("zone5x", true, false), 5);
    assert_eq!(nf.find_first_number("one5xy", true, false), 5);
    assert_eq!(nf.find_first_number("zone5xy", true, false), 5);
}

fn calibration_value(nf: &NumberFinder, s: &str, recognize_spelled: bool) -> u16 {
    let first_digit = nf.find_first_number(s, false, recognize_spelled);
    let last_digit = nf.find_first_number(s, true, recognize_spelled);

    first_digit * 10 + last_digit
}

fn part1(input: &str, nf: &NumberFinder, expected_sum: u16) {
    println!("Part 1:");

    let mut sum = 0;
    for line in input.lines() {
        sum += calibration_value(nf, line, false);
    }

    println!("Sum: {sum}");
    assert_eq!(sum, expected_sum);
}

fn part2(input: &str, nf: &NumberFinder, expected_sum: u16) {
    println!("Part 2:");

    let mut sum = 0;
    for line in input.lines() {
        sum += calibration_value(nf, line, true);
    }

    println!("Sum: {sum}");
    assert_eq!(sum, expected_sum);
}

#[test]
fn example() {
    let nf = NumberFinder::new();

    // Part 1.
    static INPUT1: &'static str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    part1(INPUT1, &nf, 142);

    // Part 2.
    static INPUT2: &'static str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    part2(INPUT2, &nf, 281);
}

fn main() {
    static INPUT: &'static str = include_str!("../input");

    let nf = NumberFinder::new();

    // Part 1
    part1(INPUT, &nf, 55_607);

    // Part 2
    part2(INPUT, &nf, 55_291);
}
