use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct WorryLevel(u64);

fn times_seven(old: WorryLevel) -> WorryLevel {
    WorryLevel(old.0 * 7)
}

fn square(old: WorryLevel) -> WorryLevel {
    WorryLevel(old.0 * old.0)
}

fn plus_eight(old: WorryLevel) -> WorryLevel {
    WorryLevel(old.0 + 8)
}

fn plus_four(old: WorryLevel) -> WorryLevel {
    WorryLevel(old.0 + 4)
}
fn plus_three(old: WorryLevel) -> WorryLevel {
    WorryLevel(old.0 + 3)
}

fn plus_five(old: WorryLevel) -> WorryLevel {
    WorryLevel(old.0 + 5)
}

fn plus_seven(old: WorryLevel) -> WorryLevel {
    WorryLevel(old.0 + 7)
}

fn times_three(old: WorryLevel) -> WorryLevel {
    WorryLevel(old.0 * 3)
}

#[cfg(test)]
fn times_nineteen(old: WorryLevel) -> WorryLevel {
    WorryLevel(old.0 * 19)
}

#[cfg(test)]
fn plus_six(old: WorryLevel) -> WorryLevel {
    WorryLevel(old.0 + 6)
}

#[derive(Clone)]
struct Monkey {
    items: VecDeque<WorryLevel>,
    operation: &'static dyn Fn(WorryLevel) -> WorryLevel,
    test: (u64, usize, usize),
    num_items_inspected: u64,
}

fn gcd(first: u64, second: u64) -> u64 {
    let (mut max, mut min) = (first, second);
    if min < max {
        (min, max) = (max, min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        (max, min) = (min, res);
    }
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn parse_input(s: &str) -> (Vec<Monkey>, u64) {
    let mut lines = s.lines();

    let mut monkeys = vec![];
    let mut current_lcm = 1;

    loop {
        // Monkey 0:
        //   Starting items: 79, 98
        //   Operation: new = old * 19
        //   Test: divisible by 23
        //     If true: throw to monkey 2
        //     If false: throw to monkey 3

        let line = match lines.next() {
            None => break,
            Some(line) => match line {
                "" => continue,
                line => line,
            },
        };

        let mut toks = line.split(' ');

        // Monkey 0:
        assert!(toks.next().expect("Monkey") == "Monkey");

        let _id: usize = match toks.next().expect("<n>:").strip_suffix(':') {
            None => panic!("expected trailing colon"),
            Some(num) => num.parse().expect("number"),
        };

        //   Starting items: 74, 64, 74, 63, 53
        let starting_items = lines
            .next()
            .expect("starting items line")
            .strip_prefix("  Starting items: ")
            .expect("starting items")
            .split(", ")
            .map(|s| WorryLevel(s.parse().expect("worry level")))
            .collect();

        //   Operation: new = old * 7
        let op: &dyn Fn(WorryLevel) -> WorryLevel = match lines
            .next()
            .expect("operation line")
            .strip_prefix("  Operation: new = ")
            .expect("operation prefix")
        {
            "old * 7" => &times_seven,
            "old * old" => &square,
            "old + 8" => &plus_eight,
            "old + 4" => &plus_four,
            "old + 3" => &plus_three,
            "old + 5" => &plus_five,
            "old + 7" => &plus_seven,
            "old * 3" => &times_three,
            #[cfg(test)]
            "old * 19" => &times_nineteen,
            #[cfg(test)]
            "old + 6" => &plus_six,
            s => panic!("unknown operation: {}", s),
        };

        //   Test: divisible by 5
        let test_divisor = lines
            .next()
            .expect("test line")
            .strip_prefix("  Test: divisible by ")
            .expect("test prefix")
            .parse()
            .expect("divisor");

        current_lcm = lcm(test_divisor, current_lcm);

        //     If true: throw to monkey 1
        let if_true = lines
            .next()
            .expect("if true")
            .strip_prefix("    If true: throw to monkey ")
            .expect("if true prefix")
            .parse()
            .expect("true monkey");

        //     If false: throw to monkey 6
        let if_false = lines
            .next()
            .expect("if false")
            .strip_prefix("    If false: throw to monkey ")
            .expect("if false prefix")
            .parse()
            .expect("false monkey");

        monkeys.push(Monkey {
            //id,
            items: starting_items,
            operation: op,
            test: (test_divisor, if_true, if_false),
            num_items_inspected: 0,
        });
    }

    (monkeys, current_lcm)
}

fn run_round(monkeys: &mut Vec<Monkey>, lcm: Option<u64>) {
    for j in 0..monkeys.len() {
        loop {
            // Remove first item from current monkey and inspect it.
            let mut item = match monkeys[j].items.pop_front() {
                None => break,
                Some(item) => item,
            };

            monkeys[j].num_items_inspected += 1;

            // Worry level changes per the monkey in question.
            item = (monkeys[j].operation)(item);

            if let Some(lcm) = lcm {
                // Confine to modular space.
                item = WorryLevel(item.0 % lcm);
            } else {
                // Or feel relief that it wasn't damaged.
                item = WorryLevel(item.0 / 3);
            }

            // Throw to new monkey.
            let test = item.0 % monkeys[j].test.0 == 0;
            let new_monkey = if test {
                monkeys[j].test.1
            } else {
                monkeys[j].test.2
            };

            monkeys[new_monkey].items.push_back(item);
        }
    }
}

fn print_inspections(monkeys: &Vec<Monkey>) {
    for (i, monkey) in monkeys.iter().enumerate() {
        println!(
            "Monkey {} inspected items {} times",
            i, monkey.num_items_inspected
        );
    }
}

fn monkey_business(monkeys: &Vec<Monkey>) -> u64 {
    let mut nums_inspected = monkeys
        .iter()
        .map(|m| m.num_items_inspected)
        .collect::<Vec<_>>();
    nums_inspected.sort_by(|a, b| b.cmp(a));
    let monkey_business = nums_inspected.iter().take(2).product::<u64>();
    println!("monkey business: {}", monkey_business);
    monkey_business
}

#[cfg(test)]
fn assert_inspection_counts(monkeys: &Vec<Monkey>, counts: &Vec<u64>) {
    let actual = monkeys
        .iter()
        .map(|m| m.num_items_inspected)
        .collect::<Vec<_>>();
    assert_eq!(actual, *counts);
}

#[test]
fn test_example() {
    let test_input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    let (monkeys, lcm) = parse_input(test_input);

    {
        let mut part1 = monkeys.clone();
        run_round(&mut part1, None);

        assert_eq!(
            part1[0]
                .items
                .iter()
                .copied()
                .map(|w| w.0)
                .collect::<Vec<_>>(),
            vec![20, 23, 27, 26]
        );

        assert_eq!(
            part1[1]
                .items
                .iter()
                .copied()
                .map(|w| w.0)
                .collect::<Vec<_>>(),
            vec![2080, 25, 167, 207, 401, 1046]
        );

        for _ in 1..20 {
            run_round(&mut part1, None);
        }

        print_inspections(&part1);

        assert_inspection_counts(&part1, &vec![101, 95, 7, 105]);
    }

    {
        let mut part2 = monkeys.clone();
        run_round(&mut part2, Some(lcm));
        assert_inspection_counts(&part2, &vec![2, 4, 3, 6]);

        for _ in 1..20 {
            run_round(&mut part2, Some(lcm));
        }

        print_inspections(&part2);
        assert_inspection_counts(&part2, &vec![99, 97, 8, 103]);

        for _ in 20..1_000 {
            run_round(&mut part2, Some(lcm));
        }

        print_inspections(&part2);
        assert_inspection_counts(&part2, &vec![5204, 4792, 199, 5192]);

        for _ in 1_000..2_000 {
            run_round(&mut part2, Some(lcm));
        }

        print_inspections(&part2);
        assert_inspection_counts(&part2, &vec![10419, 9577, 392, 10391]);

        for _ in 2_000..3_000 {
            run_round(&mut part2, Some(lcm));
        }

        print_inspections(&part2);
        assert_inspection_counts(&part2, &vec![15638, 14358, 587, 15593]);

        for _ in 3_000..4_000 {
            run_round(&mut part2, Some(lcm));
        }

        print_inspections(&part2);
        assert_inspection_counts(&part2, &vec![20858, 19138, 780, 20797]);

        for _ in 4_000..5_000 {
            run_round(&mut part2, Some(lcm));
        }

        print_inspections(&part2);
        assert_inspection_counts(&part2, &vec![26075, 23921, 974, 26000]);

        for _ in 5_000..6_000 {
            run_round(&mut part2, Some(lcm));
        }

        print_inspections(&part2);
        assert_inspection_counts(&part2, &vec![31294, 28702, 1165, 31204]);

        for _ in 6_000..7_000 {
            run_round(&mut part2, Some(lcm));
        }

        print_inspections(&part2);
        assert_inspection_counts(&part2, &vec![36508, 33488, 1360, 36400]);

        for _ in 7_000..8_000 {
            run_round(&mut part2, Some(lcm));
        }

        print_inspections(&part2);
        assert_inspection_counts(&part2, &vec![41728, 38268, 1553, 41606]);

        for _ in 8_000..9_000 {
            run_round(&mut part2, Some(lcm));
        }

        print_inspections(&part2);
        assert_inspection_counts(&part2, &vec![46945, 43051, 1746, 46807]);

        for _ in 9_000..10_000 {
            run_round(&mut part2, Some(lcm));
        }

        print_inspections(&part2);
        assert_inspection_counts(&part2, &vec![52166, 47830, 1938, 52013]);

        let mb = monkey_business(&part2);
        assert_eq!(mb, 2713310158);
    }
}

fn main() {
    let (monkeys, lcm) = parse_input(include_str!("../input"));
    println!("LCM: {}", lcm);

    // Part 1.
    println!("Part 1:");
    let mut part1 = monkeys.clone();
    for _ in 0..20 {
        run_round(&mut part1, None);
    }
    print_inspections(&part1);

    let mb = monkey_business(&part1);
    assert_eq!(mb, 54_054);

    // Part 2.
    println!("Part 2:");
    let mut part2 = monkeys.clone();
    for _ in 0..10_000 {
        run_round(&mut part2, Some(lcm));
    }
    print_inspections(&part2);

    monkey_business(&part2);
}
