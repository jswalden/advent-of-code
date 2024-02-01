use itertools::Itertools;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Spring {
    Working,
    Damaged,
    Unknown,
}

pub struct ConditionRecord {
    springs: Vec<Spring>,
    damaged_run_lengths: Vec<usize>,
}

impl ConditionRecord {
    fn new(s: &str) -> ConditionRecord {
        let (springs, damaged_run_lengths) = s.split_once(' ').expect("record");

        let springs = springs
            .chars()
            .map(|c| match c {
                '.' => Spring::Working,
                '#' => Spring::Damaged,
                '?' => Spring::Unknown,
                c => panic!("bad spring: {c:?}"),
            })
            .collect();

        let damaged_run_lengths = damaged_run_lengths
            .split(',')
            .map(|n| n.parse::<usize>().expect("n"))
            .collect();

        ConditionRecord {
            springs,
            damaged_run_lengths,
        }
    }

    fn count_valid_arrangements(&self) -> u64 {
        let total_spring_count = self.springs.len();
        let damaged_sequence_count = self.damaged_run_lengths.len();

        let mut incremental = vec![vec![0u64; total_spring_count + 2]; damaged_sequence_count + 1];
        incremental[damaged_sequence_count][total_spring_count + 1] = 1;

        // Consider each damaged-length from last to first.
        for (d, damaged_len) in self.damaged_run_lengths.iter().copied().enumerate().rev() {
            let mut possibly_damaged_run_len = 0;

            let mut nways = 0;

            // Attempt to place a damaged-length at each possible location from end of spring sequence to start.
            for s in (0..total_spring_count).rev() {
                nways = if let Some(Spring::Damaged) = self.springs.get(s + damaged_len) {
                    0
                } else {
                    match incremental[d + 1].get(s + damaged_len + 1) {
                        Some(ways) => nways + *ways,
                        None => 0,
                    }
                };

                incremental[d][s] = match self.springs[s] {
                    Spring::Working => {
                        possibly_damaged_run_len = 0;
                        0
                    }
                    Spring::Unknown | Spring::Damaged => {
                        possibly_damaged_run_len += 1;

                        if possibly_damaged_run_len >= damaged_len
                            && (s == 0 || self.springs[s - 1] != Spring::Damaged)
                            && {
                                let limit = s + damaged_len;
                                limit == total_spring_count
                                    || self.springs[limit] != Spring::Damaged
                            }
                        {
                            nways
                        } else {
                            0
                        }
                    }
                };
            }
        }

        println!("incremental:");
        for row in incremental.iter() {
            println!("{row:?}");
        }

        let ans = Itertools::take_while_inclusive(
            incremental[0].iter().take(total_spring_count).enumerate(),
            |(s, _)| self.springs[*s] != Spring::Damaged,
        )
        .map(|(_s, ways)| *ways)
        .sum();

        println!("Computed answer: {ans}");

        ans
    }
}

mod tests;

pub fn parse_input(input: &'static str) -> impl Iterator<Item = ConditionRecord> {
    input.trim().lines().map(|line| ConditionRecord::new(line))
}

pub fn parse_input_repeated(input: &'static str) -> impl Iterator<Item = ConditionRecord> {
    input
        .trim()
        .lines()
        .map(|line| ConditionRecord::new(line))
        .map(
            |ConditionRecord {
                 springs: ref single_springs,
                 damaged_run_lengths: ref single_damaged_run_lengths,
             }| {
                const N: usize = 5;

                let springs = {
                    let mut springs = single_springs.clone();
                    for _ in 0..N - 1 {
                        springs.push(Spring::Unknown);
                        springs.extend_from_slice(single_springs);
                    }
                    springs
                };

                let damaged_run_lengths = {
                    let mut damaged_run_lengths = vec![];
                    for _ in 0..N {
                        damaged_run_lengths.extend_from_slice(single_damaged_run_lengths);
                    }
                    damaged_run_lengths
                };

                ConditionRecord {
                    springs,
                    damaged_run_lengths,
                }
            },
        )
}

fn main() {
    static INPUT: &str = include_str!("../input");

    // Part 1.
    println!("Part 1");
    let sum: u64 = parse_input(INPUT)
        .map(|rec| rec.count_valid_arrangements())
        .sum();
    println!("Sum: {sum}");
    assert_eq!(sum, 7307);

    // Part 2.
    println!("Part 2");
    let sum: u64 = parse_input_repeated(INPUT)
        .map(|rec| rec.count_valid_arrangements())
        .sum();
    println!("Sum: {sum}");
    assert_eq!(sum, 3_415_570_893_842);
}
