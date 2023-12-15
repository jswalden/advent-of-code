use itertools::Itertools;

fn find_next_value(nums: &Vec<i32>) -> i32 {
    let mut all_zeroes = true;
    let diffs = Itertools::tuple_windows(nums.iter())
        .map(|(x, y)| {
            let diff = y - x;
            all_zeroes = all_zeroes && diff == 0;
            diff
        })
        .collect();

    let end = *nums.last().expect("last");
    if all_zeroes {
        end
    } else {
        end + find_next_value(&diffs)
    }
}

#[test]
fn next_value_test() {
    assert_eq!(find_next_value(&vec![0, 0, 0, 0]), 0);
    assert_eq!(find_next_value(&vec![3, 3, 3, 3]), 3);
    assert_eq!(find_next_value(&vec![0, 3, 6, 9]), 12);
    assert_eq!(find_next_value(&vec![1, 3, 6, 10, 15, 21]), 28);
}

fn parse_values(line: &str) -> Vec<i32> {
    line.split_ascii_whitespace()
        .map(|n| n.parse::<i32>().expect("n"))
        .collect()
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(parse_values).collect()
}

#[test]
fn example() {
    static INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    let parsed = parse_input(INPUT);

    // Part 1.
    println!("Part 1:");
    let sum_nexts: i32 = parsed.iter().map(|num| find_next_value(num)).sum();
    println!("Sum: {sum_nexts}");
    assert_eq!(sum_nexts, 114);

    // Part 2.
    println!("Part 2:");
    let reversed: Vec<Vec<i32>> = parsed
        .iter()
        .cloned()
        .map(|mut v| {
            v.reverse();
            v
        })
        .collect();
    let sum_prevs: i32 = reversed.iter().map(|num| find_next_value(num)).sum();
    println!("Sum (backward): {sum_prevs}");
    assert_eq!(sum_prevs, 2);
}

fn main() {
    static INPUT: &str = include_str!("../input");

    let parsed = parse_input(INPUT);

    // Part 1.
    println!("Part 1:");
    let sum_nexts: i32 = parsed.iter().map(|num| find_next_value(num)).sum();
    println!("Sum (forward): {sum_nexts}");
    assert_eq!(sum_nexts, 1_853_145_119);

    // Part 2.
    println!("Part 2:");
    let reversed: Vec<Vec<i32>> = parsed
        .iter()
        .cloned()
        .map(|mut v| {
            v.reverse();
            v
        })
        .collect();
    let sum_prevs: i32 = reversed.iter().map(|num| find_next_value(num)).sum();
    println!("Sum (backward): {sum_prevs}");
    assert_eq!(sum_prevs, 42);
}
