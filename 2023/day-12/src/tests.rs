#![cfg(test)]

use crate::{parse_input, ConditionRecord};

fn line_to_valid_arrangements(line: &str) -> u64 {
    println!("-------------------------\nConsidering: {line}");
    let cr = ConditionRecord::new(line);
    cr.count_valid_arrangements()
}

#[test]
fn power_of_two() {
    assert_eq!(line_to_valid_arrangements("??.??.?? 1,1,1"), 8);
}

#[test]
fn ones() {
    assert_eq!(line_to_valid_arrangements("# 1"), 1);
    assert_eq!(line_to_valid_arrangements("? 1"), 1);
}

#[test]
fn twos() {
    assert_eq!(line_to_valid_arrangements("?? 2"), 1);
    assert_eq!(line_to_valid_arrangements("#? 2"), 1);
    assert_eq!(line_to_valid_arrangements("?# 2"), 1);
    assert_eq!(line_to_valid_arrangements("?? 1"), 2);
    assert_eq!(line_to_valid_arrangements("#? 1"), 1);
    assert_eq!(line_to_valid_arrangements("?# 1"), 1);
}

#[test]
fn threes() {
    assert_eq!(line_to_valid_arrangements("#.. 1"), 1);
    assert_eq!(line_to_valid_arrangements(".#. 1"), 1);
    assert_eq!(line_to_valid_arrangements("..# 1"), 1);
    assert_eq!(line_to_valid_arrangements("#?. 1"), 1);
    assert_eq!(line_to_valid_arrangements("#.? 1"), 1);
    assert_eq!(line_to_valid_arrangements("?#. 1"), 1);
    assert_eq!(line_to_valid_arrangements(".#? 1"), 1);
    assert_eq!(line_to_valid_arrangements("?.# 1"), 1);
    assert_eq!(line_to_valid_arrangements(".?# 1"), 1);

    assert_eq!(line_to_valid_arrangements("?#? 1"), 1);
    assert_eq!(line_to_valid_arrangements("#?? 1"), 1);

    assert_eq!(line_to_valid_arrangements("#?? 1"), 1);
    assert_eq!(line_to_valid_arrangements("##? 2"), 1);
}

#[test]
fn must_undo_one() {
    assert_eq!(line_to_valid_arrangements("???..?#?..?? 2"), 2);
}

#[test]
fn must_undo_two() {
    assert_eq!(line_to_valid_arrangements("???..?#?..?? 2,2"), 6);
}

#[test]
fn longer_stretch_start_undo() {
    assert_eq!(line_to_valid_arrangements("#??..? 1"), 1);
}

#[test]
fn longer_stretch_middle_undo() {
    assert_eq!(line_to_valid_arrangements("?#?..? 1"), 1);
}

#[test]
fn various_attempts() {
    assert_eq!(line_to_valid_arrangements("?#?..?..? 1"), 1);
    assert_eq!(line_to_valid_arrangements("???#...#.?#??#?#? 1,1,1,8"), 2);

    // assert_eq!(
    //     line_to_valid_arrangements("?.??????#?????..#? 1,2,1,1,1,2"),
    //     1
    // );

    assert_eq!(line_to_valid_arrangements("# 1"), 1);
}

#[test]
fn longer_stretch_end_undo() {
    assert_eq!(line_to_valid_arrangements("??#..? 1"), 1);
}

#[test]
fn must_undo_three() {
    assert_eq!(line_to_valid_arrangements("???..?#?..?? 2,2,2"), 4);
}

#[test]
fn single() {
    assert_eq!(line_to_valid_arrangements("??# 1"), 1);
}

#[test]
fn all_unknown_one() {
    assert_eq!(line_to_valid_arrangements("??? 1"), 3);
}

#[test]
fn first_example_line() {
    assert_eq!(line_to_valid_arrangements("???.### 1,1,3"), 1);
}

#[test]
fn second_example_line() {
    assert_eq!(line_to_valid_arrangements(".??..??...?##. 1,1,3"), 4);
}

#[test]
fn third_example_line() {
    assert_eq!(line_to_valid_arrangements("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
}

#[test]
fn fourth_example_line() {
    assert_eq!(line_to_valid_arrangements("????.#...#... 4,1,1"), 1);
}

#[test]
fn fifth_example_line() {
    assert_eq!(line_to_valid_arrangements("????.######..#####. 1,6,5"), 4);
}

#[test]
fn sixth_example_line() {
    assert_eq!(line_to_valid_arrangements("?###???????? 3,2,1"), 10);
}

#[test]
fn example() {
    static INPUT: &str = "
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
    ";

    // Part 1.
    println!("Part 1");
    let sum: u64 = parse_input(INPUT)
        .map(|rec| rec.count_valid_arrangements())
        .sum();
    println!("Sum: {sum}");
    assert_eq!(sum, 21);
}
