use std::collections::HashSet;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Sensor(i32, i32);
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Beacon(i32, i32);

type SensorBeaconVec = Vec<(Sensor, Beacon)>;

fn parse_input(s: &str) -> SensorBeaconVec {
    let sensor_at = "Sensor at x=";
    let sensor_at_len = sensor_at.len();

    let comma_y_eq = ", y=";
    let comma_y_eq_len = comma_y_eq.len();

    let closest_beacon_is_at_x_eq = ": closest beacon is at x=";
    let closest_beacon_is_at_x_eq_len = closest_beacon_is_at_x_eq.len();

    let sensor_and_beacon = |line: &str| {
        // Sensor at x=13820, y=3995710: closest beacon is at x=1532002, y=3577287
        let mut comma_y_iter = line.match_indices(comma_y_eq);

        let sensor_x = (
            sensor_at_len,
            comma_y_iter.next().expect("between sensor x/y").0,
        );
        let sensor_y = (
            sensor_x.1 + comma_y_eq_len,
            line.find(closest_beacon_is_at_x_eq)
                .expect("before beacon x"),
        );

        let beacon_x = (
            sensor_y.1 + closest_beacon_is_at_x_eq_len,
            comma_y_iter.next().expect("between beacon x/y").0,
        );
        let beacon_y = (beacon_x.1 + comma_y_eq_len, line.len());

        let sensor = Sensor(
            line[sensor_x.0..sensor_x.1].parse().expect("sensor x"),
            line[sensor_y.0..sensor_y.1].parse().expect("sensor y"),
        );

        let beacon = Beacon(
            line[beacon_x.0..beacon_x.1].parse().expect("beacon x"),
            line[beacon_y.0..beacon_y.1].parse().expect("beacon y"),
        );

        (sensor, beacon)
    };

    s.lines().map(sensor_and_beacon).collect()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ExcludedRange(i32, i32);

fn find_excluded_range((sensor, beacon): &(Sensor, Beacon), row: i32) -> Option<ExcludedRange> {
    let x_steps = (sensor.0 - beacon.0).abs();
    let y_steps = (sensor.1 - beacon.1).abs();
    let total_steps = x_steps + y_steps;

    let y_delta = (sensor.1 - row).abs();
    if y_delta > total_steps {
        return None;
    }

    let remaining = total_steps - y_delta;
    Some(ExcludedRange(sensor.0 - remaining, sensor.0 + remaining))
}

#[test]
fn test_find_excluded_range() {
    let pair = (Sensor(8, 7), Beacon(2, 10));

    assert_eq!(find_excluded_range(&pair, 17), None);
    assert_eq!(find_excluded_range(&pair, 16), Some(ExcludedRange(8, 8)));
    assert_eq!(find_excluded_range(&pair, 15), Some(ExcludedRange(7, 9)));
    assert_eq!(find_excluded_range(&pair, -3), None);
    assert_eq!(find_excluded_range(&pair, -2), Some(ExcludedRange(8, 8)));
    assert_eq!(find_excluded_range(&pair, -1), Some(ExcludedRange(7, 9)));
    assert_eq!(find_excluded_range(&pair, 7), Some(ExcludedRange(-1, 17)));
}

fn collapse_ranges(ranges: Vec<ExcludedRange>) -> Vec<ExcludedRange> {
    let mut collapsed = vec![];

    let mut range_iter = ranges.iter();

    let mut current_start;
    let mut current_end;

    match range_iter.next() {
        None => return collapsed,
        Some(range) => {
            ExcludedRange(current_start, current_end) = *range;
        }
    }

    for ExcludedRange(start, end) in range_iter {
        assert!(start <= end);

        if *start <= current_end {
            current_end = current_end.max(*end);
            continue;
        } else {
            collapsed.push(ExcludedRange(current_start, current_end));
        }

        current_start = *start;
        current_end = *end;
    }

    collapsed.push(ExcludedRange(current_start, current_end));

    collapsed
}

#[test]
fn test_collapse_ranges() {
    assert_eq!(
        collapse_ranges(vec![ExcludedRange(0, 0)]),
        vec![ExcludedRange(0, 0)]
    );
    assert_eq!(
        collapse_ranges(vec![ExcludedRange(2, 2)]),
        vec![ExcludedRange(2, 2)]
    );
    assert_eq!(
        collapse_ranges(vec![ExcludedRange(2, 4)]),
        vec![ExcludedRange(2, 4)]
    );
    assert_eq!(
        collapse_ranges(vec![ExcludedRange(2, 5), ExcludedRange(3, 4)]),
        vec![ExcludedRange(2, 5)]
    );
    assert_eq!(
        collapse_ranges(vec![ExcludedRange(2, 5), ExcludedRange(3, 6)]),
        vec![ExcludedRange(2, 6)]
    );
}

fn count_size_of_ranges(ranges: Vec<ExcludedRange>) -> u32 {
    let mut count = 0;

    let mut prev_end = None;
    for range in &ranges {
        let ExcludedRange(start, end) = *range;
        assert!(start <= end);

        if let Some(prev_end) = prev_end {
            assert!(prev_end < start);
        }

        count += (end - start) as u32 + 1;
        prev_end = Some(end);
    }
    count
}

fn excluded_ranges_in_row(sensors_beacons: &SensorBeaconVec, row: i32) -> Vec<ExcludedRange> {
    let mut ranges: Vec<_> = sensors_beacons
        .iter()
        .filter_map(|sensor_beacon| find_excluded_range(sensor_beacon, row))
        .collect();
    ranges.sort();

    collapse_ranges(ranges)
}

fn count_excluded_positions_in_row(sensors_beacons: &SensorBeaconVec, row: i32) -> u32 {
    let collapsed_ranges = excluded_ranges_in_row(sensors_beacons, row);

    let beacons_in_row = sensors_beacons
        .iter()
        .filter_map(|(_, beacon)| if row == beacon.1 { Some(*beacon) } else { None })
        .collect::<HashSet<_>>()
        .iter()
        .count();

    count_size_of_ranges(collapsed_ranges) - beacons_in_row as u32
}

fn find_permissible_position(
    sensors_beacons: &SensorBeaconVec,
    max_x: i32,
    max_y: i32,
) -> Option<(i32, i32)> {
    for row in 0..=max_y {
        let ranges = excluded_ranges_in_row(sensors_beacons, row);

        let mut prev_end = -1;
        for ExcludedRange(start, end) in ranges
            .iter()
            .filter(|ExcludedRange(start, end)| {
                !((*start < 0 && *end < 0) || (max_x < *start && max_x < *end))
            })
            .copied()
        {
            if prev_end + 1 < start && 0 <= prev_end + 1 {
                return Some((prev_end + 1, row));
            }
            prev_end = end;
        }

        if prev_end < max_x {
            return Some((max_x, row));
        }
    }

    panic!("distress beacon not found");
}

fn tuning_frequency(x: i32, y: i32) -> i64 {
    x as i64 * 4_000_000 + y as i64
}

#[test]
fn test_example() {
    let example = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    let sensors_beacons = parse_input(example);

    {
        const ROW: i32 = 10;
        let count = count_excluded_positions_in_row(&sensors_beacons, ROW);
        println!("excluded positions in row {}: {}", ROW, count);
        assert_eq!(count, 26);
    }

    {
        const MAX_X: i32 = 20;
        const MAX_Y: i32 = 20;

        if let Some((x, y)) = find_permissible_position(&sensors_beacons, MAX_X, MAX_Y) {
            println!("permissible position is ({}, {})", x, y);
            assert_eq!((x, y), (14, 11));

            let tf = tuning_frequency(x, y);
            println!("tuning frequency: {}", tf);
            assert_eq!(tf, 56_000_011);
        } else {
            panic!("no distress beacon found");
        }
    }
}

fn main() {
    let input = include_str!("../input");

    let sensors_beacons = parse_input(input);

    {
        const ROW: i32 = 2_000_000;
        let count = count_excluded_positions_in_row(&sensors_beacons, ROW);
        println!("excluded positions in row {}: {}", ROW, count);
    }

    {
        const MAX_X: i32 = 4_000_000;
        const MAX_Y: i32 = 4_000_000;

        if let Some((x, y)) = find_permissible_position(&sensors_beacons, MAX_X, MAX_Y) {
            println!("permissible position is ({}, {})", x, y);
            //assert_eq!((x, y), (14, 11));

            let tf = tuning_frequency(x, y);
            println!("tuning frequency: {}", tf);
            //assert_eq!(tf, 56_000_011);
        } else {
            panic!("no distress beacon found");
        }
    }
}
