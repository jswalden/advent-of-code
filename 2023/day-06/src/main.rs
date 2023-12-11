#[derive(Debug)]
struct Race {
    time: u64,
    current_record: u64,
}

impl Race {
    fn ways_to_beat(&self) -> u64 {
        println!("");
        println!("Time:   {time}", time = self.time);
        println!("Record: {record}", record = self.current_record);

        let t_sq = self.time.pow(2);
        let four_a_c = 4 * self.current_record;
        let root = (t_sq - four_a_c) as f64;

        let sqrt = root.sqrt();

        let mut lowest = (self.time as f64 - sqrt) / 2.0;
        let mut highest = (self.time as f64 + sqrt) / 2.0;
        println!("lowest: {lowest}, highest: {highest}");

        if lowest * (self.time as f64 - lowest) <= self.current_record as f64 {
            lowest = lowest.floor() + 1.0;
        } else {
            lowest = lowest.ceil();
        }

        if highest * (self.time as f64 - highest) >= self.current_record as f64 {
            highest = highest.ceil() - 1.0;
        } else {
            highest = highest.floor();
        }

        println!("lowest (adjusted): {lowest}, highest (adjusted): {highest}");
        (highest - lowest + 1.0) as u64
    }
}

#[test]
fn ways_to_beat() {
    assert_eq!(
        Race {
            time: 7,
            current_record: 9
        }
        .ways_to_beat(),
        4
    );

    assert_eq!(
        Race {
            time: 15,
            current_record: 40
        }
        .ways_to_beat(),
        8
    );

    assert_eq!(
        Race {
            time: 30,
            current_record: 200
        }
        .ways_to_beat(),
        9
    );
}

// 9 = -D**2 + T*D
// 0 = -D**2 + T*D - 9
// 0 = D**2 - T*D + 9

// T/2 +- sqrt(T**2 - 4*CR)/2
// sqrt(49 - 4*9) = sqrt(13)

// 12 = -9 + 7*3
// CR = -D**2 + T*D
// 0 = -D**2 + T*D - CR

// CR = T*D - D**2

// 0 = D**2 -T*D + CR

// D = T/2 +- sqrt(T**2 - 4*CR)

// sqrt(time**2 - 4 * current_record)

// D = T/2 - sqrt(T**2 - 4*CR)
// D = T/2 + sqrt(T**2 - 4*CR)
// D = 3.5 - sqrt(13)
// D = 3.5 + sqrt(13)

/*
Time:        44     89     96     91
Distance:   277   1136   1890   1768
 */

// 44/2 +- sqrt(44**2 - 4*277)/2
// 22 +- 28.77

// 89/2 +- sqrt(89**2 - 4*1136)/2
// 44.5 +- 58.111

// 96/2 +- sqrt(96**2 - 4*1890)/2
// 48 +- 40.69

// 91/2 +- sqrt(91**2 - 4*1768)/2
// 45.5 +- 34.77

fn parse_input(s: &str) -> Vec<Race> {
    let mut lines = s.lines();

    let mut times = lines.next().expect("times").split_ascii_whitespace();
    let mut records = lines.next().expect("records").split_ascii_whitespace();

    times.next();
    records.next();

    let mut races = vec![];

    loop {
        match (times.next(), records.next()) {
            (None, None) => break,
            (Some(time), Some(record)) => races.push(Race {
                time: time.parse::<u64>().expect("time"),
                current_record: record.parse::<u64>().expect("record"),
            }),
            _ => panic!("time/distance count mismatch"),
        }
    }

    races
}

fn count_decimal_digits(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        (n as f64 + 0.1).log10().ceil() as u64
    }
}

#[test]
fn count_digits() {
    assert_eq!(count_decimal_digits(0), 1);
    assert_eq!(count_decimal_digits(1), 1);
    assert_eq!(count_decimal_digits(9), 1);
    assert_eq!(count_decimal_digits(10), 2);
    assert_eq!(count_decimal_digits(11), 2);
    assert_eq!(count_decimal_digits(99), 2);
    assert_eq!(count_decimal_digits(100), 3);
    assert_eq!(count_decimal_digits(101), 3);
    assert_eq!(count_decimal_digits(999), 3);
    assert_eq!(count_decimal_digits(1000), 4);
}

fn races_to_race(races: &Vec<Race>) -> Race {
    let (time, current_record) = races
        .iter()
        .map(
            |&Race {
                 time,
                 current_record,
             }| { (time, current_record) },
        )
        .fold(
            (0, 0),
            |(full_time, full_record), (time, current_record)| {
                let digits_time = count_decimal_digits(time);
                let digits_record = count_decimal_digits(current_record);
                (
                    full_time * 10u64.pow(digits_time as u32) + time,
                    full_record * 10u64.pow(digits_record as u32) + current_record,
                )
            },
        );

    Race {
        time,
        current_record,
    }
}

#[test]
fn example() {
    static INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    let races = parse_input(INPUT);

    // Part 1.
    println!("Part 1:");
    let prod = races
        .iter()
        .map(|r| {
            let wtb = r.ways_to_beat();
            println!("Ways to beat: {wtb}");
            wtb
        })
        .product::<u64>();
    println!("Product: {prod}");
    assert_eq!(prod, 288);

    // Part 2.
    println!("Part 2:");
    let single_race = races_to_race(&races);
    println!("Single race: {single_race:?}");
    let ways = single_race.ways_to_beat();
    println!("Ways to beat: {ways}");
    assert_eq!(ways, 71503);
}

fn main() {
    static INPUT: &str = include_str!("../input");

    let races = parse_input(INPUT);

    // Part 1.
    println!("Part 1:");
    let prod = races
        .iter()
        .map(|r| {
            let wtb = r.ways_to_beat();
            println!("Ways to beat: {wtb}");
            wtb
        })
        .product::<u64>();
    println!("Product: {prod}");
    assert_eq!(prod, 2344708);

    // Part 2.
    println!("Part 2:");
    let single_race = races_to_race(&races);
    println!("Single race: {single_race:?}");
    let ways = single_race.ways_to_beat();
    println!("Ways to beat: {ways}");
    //assert_eq!(ways, 42);
}
