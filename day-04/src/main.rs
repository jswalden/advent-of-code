static CONTENTS: &str = include_str!("../input");

fn part1() {
    let count: u32 = CONTENTS
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(',').expect("two ranges");

            let to_range = |(x, y): (&str, &str)| {
                (
                    x.parse::<u32>().expect("range start"),
                    y.parse::<u32>().expect("range end"),
                )
            };

            let (first_start, first_end) =
                first.split_once('-').map(to_range).expect("first range");

            let (second_start, second_end) =
                second.split_once('-').map(to_range).expect("second range");

            if (first_start <= second_start && second_end <= first_end)
                || (second_start <= first_start && first_end <= second_end)
            {
                1
            } else {
                0
            }
        })
        .sum();

    print!("total nested ranges: {}\n", count);
}

fn part2() {
    let count: u32 = CONTENTS
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(',').expect("two ranges");

            let to_range = |(x, y): (&str, &str)| {
                (
                    x.parse::<u32>().expect("range start"),
                    y.parse::<u32>().expect("range end"),
                )
            };

            let (first_start, first_end) =
                first.split_once('-').map(to_range).expect("first range");

            let (second_start, second_end) =
                second.split_once('-').map(to_range).expect("second range");

            if (first_start <= second_end && second_start <= first_end)
                || (second_start <= first_end && first_start <= second_end)
            {
                1
            } else {
                0
            }
        })
        .sum();

    print!("total overlapping ranges: {}\n", count);
}

fn main() {
    part1();
    part2();
}
