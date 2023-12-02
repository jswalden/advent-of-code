use itertools::Itertools;

fn count_larger_measurements(report: &str) -> usize {
    Itertools::tuple_windows(report.lines().map(|line| line.parse::<u32>().unwrap()))
        .fold(0, |count, (x, y)| count + (x < y) as usize)
}

fn count_sliding_sum_increases(report: &str) -> usize {
    Itertools::tuple_windows(
        Itertools::tuple_windows(report.lines().map(|line| line.parse::<u32>().unwrap()))
            .map(|(a, b, c)| a + b + c),
    )
    .fold(0, |count, (sum, next_sum)| {
        count + (sum < next_sum) as usize
    })
}

#[test]
fn example() {
    static INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

    // Part 1.
    let larger_measurements = count_larger_measurements(INPUT);
    println!("Test part 1 larger measurements: {larger_measurements}");
    assert_eq!(larger_measurements, 7);

    // Part 2.
    let sum_increases = count_sliding_sum_increases(INPUT);
    println!("Test part 2 larger window sums: {sum_increases}");
    assert_eq!(sum_increases, 5);
}

fn main() {
    static INPUT: &str = include_str!("../input");

    // Part 1.
    let larger_measurements = count_larger_measurements(INPUT);
    println!("Part 1 larger measurements: {larger_measurements}");
    assert_eq!(larger_measurements, 1655);

    // Part 2.
    let sum_increases = count_sliding_sum_increases(INPUT);
    println!("Part 2 larger window sums: {sum_increases}");
    assert_eq!(sum_increases, 1683);
}
