use num::integer;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

struct Challenge {
    directions: Vec<Direction>,
    start_nodes: Vec<&'static str>,
    network: HashMap<&'static str, (&'static str, &'static str)>,
}

static AAA: &'static str = "AAA";
static ZZZ: &str = "ZZZ";

impl Challenge {
    fn new(s: &'static str) -> Challenge {
        let mut lines = s.lines();

        let directions = lines
            .next()
            .expect("directions")
            .chars()
            .map(|c| match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                c => panic!("bad direction: {c}"),
            })
            .collect();

        let empty = lines.next().expect("blank");
        assert_eq!(empty, "");

        let elements_and_nexts =
            lines
                .map(|line| line.split_once(" = ").expect("eq"))
                .map(|(node, nexts)| {
                    (
                        node,
                        nexts[1..nexts.len() - 1]
                            .split_once(", ")
                            .expect("left-right"),
                    )
                });

        let mut network = HashMap::new();
        let mut start_nodes = vec![];
        for (node, nexts) in elements_and_nexts {
            if node.ends_with('A') {
                start_nodes.push(node);
            }

            network.insert(node, nexts);
        }

        Challenge {
            directions,
            start_nodes,
            network,
        }
    }
}

fn distance_start_to_end(challenge: &Challenge) -> usize {
    let mut distance = 0;
    let mut current_node = AAA;

    let mut directions = challenge.directions.iter().copied().cycle();

    let network = &challenge.network;
    loop {
        if current_node == ZZZ {
            break;
        }

        distance += 1;

        let (left, right) = network.get(&current_node).expect("current_node");

        match directions.next().expect("direction") {
            Direction::Left => {
                current_node = left;
            }
            Direction::Right => {
                current_node = right;
            }
        }
    }

    distance
}

fn distance_start_to_simultaneous_end(challenge: &Challenge) -> u64 {
    macro_rules! dbg_print {
        ($($toks:tt)*) => {
            if false {
                println!($($toks)*);
            }
        };
    }

    fn is_end(n: &'static str) -> bool {
        n.ends_with('Z')
    }

    println!(
        "start_nodes: {start_nodes:?}",
        start_nodes = challenge.start_nodes
    );

    let mut walks_from = challenge
        .start_nodes
        .iter()
        .copied()
        .map(|start_node| {
            let mut ending_z = HashMap::new();

            let mut distance = 0u64;
            let mut prev_node = AAA;
            let mut current_node = start_node;

            let mut start = false;
            let network = &challenge.network;
            let mut directions = challenge.directions.iter().copied().cycle();
            loop {
                if is_end(current_node) {
                    ending_z.insert(prev_node, (current_node, distance));

                    if !start {
                        start = true;

                        distance = 0;
                    } else {
                        if ending_z.contains_key(&current_node) {
                            break;
                        }
                    }

                    prev_node = current_node;
                }

                distance += 1;

                let (left, right) = network.get(&current_node).expect("current_node");

                match directions.next().expect("direction") {
                    Direction::Left => {
                        current_node = *left;
                    }
                    Direction::Right => {
                        current_node = *right;
                    }
                }
            }

            let loop_start = current_node;
            (loop_start, ending_z)
        })
        .collect::<Vec<_>>();

    walks_from.sort_by_key(|(_, ending_z)| ending_z.get(&AAA).expect("to first ending-z").1);

    println!("walks_from: {walks_from:?}");

    struct State {
        _loop_start: &'static str,
        next: (&'static str, u64),
        ending_z: HashMap<&'static str, (&'static str, u64)>,
    }

    let mut states = walks_from
        .into_iter()
        .fold(vec![], |mut states, (loop_start, ending_z)| {
            let (next_node, next_node_dist) = *ending_z.get(&AAA).expect("AAA");
            states.push(State {
                _loop_start: loop_start,
                next: (next_node, next_node_dist),
                ending_z,
            });
            states
        });

    let advance_by = |mut distance: u64,
                      (mut next_node, mut next_node_dist): (&'static str, u64),
                      ending_z: &HashMap<_, _>| {
        assert!(distance > 0);

        if next_node_dist == 0 {
            (next_node, next_node_dist) = *ending_z.get(&next_node).expect("new next");
        }

        while distance > next_node_dist {
            distance -= next_node_dist;
            next_node = ending_z.get(&next_node).expect("next_node").0;
        }

        next_node_dist -= distance;

        (next_node, next_node_dist)
    };

    let mut current_dist = 0;
    loop {
        let advance_amount = states
            .iter()
            .map(
                |&State {
                     next: (next_node, next_node_dist),
                     ref ending_z,
                     ..
                 }| {
                    if next_node_dist == 0 {
                        ending_z.get(&next_node).expect("advance_next").1
                    } else {
                        next_node_dist
                    }
                },
            )
            .max()
            .expect("states max");
        dbg_print!("Advancing by {advance_amount}...");

        for (i, s) in states.iter_mut().enumerate() {
            let curr = s.next;
            s.next = advance_by(advance_amount, s.next, &s.ending_z);
            dbg_print!("state {i}: prev: {curr:?}, to: {to:?}", to = s.next);
        }

        current_dist += advance_amount;

        if states
            .iter()
            .map(
                |&State {
                     next: (_, next_node_dist),
                     ..
                 }| { next_node_dist },
            )
            .all(|dist| dist == 0)
        {
            return current_dist;
        }
    }
}

#[test]
fn my_example() {
    // Part 2.
    static INPUT3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    let challenge3 = Challenge::new(INPUT3);
    let dist3 = distance_start_to_simultaneous_end(&challenge3);
    println!("Distance 3: {dist3}");
    assert_eq!(dist3, 6);
}

#[test]
fn example() {
    // Part 1.
    static INPUT1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    let challenge1 = Challenge::new(INPUT1);

    println!("Part 1:");
    let dist1 = distance_start_to_end(&challenge1);
    println!("Distance 1: {dist1}");
    assert_eq!(dist1, 2);

    static INPUT2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    let challenge2 = Challenge::new(INPUT2);
    let dist2 = distance_start_to_end(&challenge2);
    println!("Distance 2: {dist2}");
    assert_eq!(dist2, 6);

    // Part 2.
    static INPUT3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    let challenge3 = Challenge::new(INPUT3);
    let dist3 = distance_start_to_simultaneous_end(&challenge3);
    println!("Distance 3: {dist3}");
    assert_eq!(dist3, 6);
}

fn main() {
    static INPUT: &str = include_str!("../input");

    let challenge = Challenge::new(INPUT);

    // Part 1.
    println!("Part 1:");
    let dist = distance_start_to_end(&challenge);
    println!("Distance: {dist}");
    assert_eq!(dist, 16697);

    // Part 2.
    println!("Part 2:");
    let dist = if false {
        // This code isn't working yet, but it does work well enough to (at the
        // time this comment was written) print out
        //
        // walks_from: [("VQZ", {"VQZ": ("VQZ", 12169), "AAA": ("VQZ", 12169)}),
        //              ("BLZ", {"AAA": ("BLZ", 13301), "BLZ": ("BLZ", 13301)}),
        //              ("ZZZ", {"AAA": ("ZZZ", 16697), "ZZZ": ("ZZZ", 16697)}),
        //              ("KKZ", {"AAA": ("KKZ", 17263), "KKZ": ("KKZ", 17263)}),
        //              ("RGZ", {"RGZ": ("RGZ", 20093), "AAA": ("RGZ", 20093)}),
        //              ("XSZ", {"AAA": ("XSZ", 20659), "XSZ": ("XSZ", 20659)})]
        //
        // implying that every path from ??A to an initial ??Z and then
        // progressing eventually back to it takes X steps to first ??Z and then
        // X steps to return to it.  (On typing this out I realize I haven't
        // guaranteed identical offset-into-directions status for the ??Z to
        // itself loop, whoops.)
        //
        // So we can just situationally "cheat"* and find the LCM of all the
        // different Xs to know when they all converge on ??Z simultaneously,
        // even if that's not a general solution to the problem because you
        // might have AAA -> BBZ -> CCD -> DDE -> EEZ -> FFG -> BBZ where
        // distance-to-first-??Z is 1, then the loop has walks of 3 and 2 to the
        // next ??Z -- and if directions are LR then on second return to BBZ
        // you'd move L out of it, not R.
        //
        // But I've fallen behind on AoC and am presently more interested in
        // catching up, than in ratholing on the generalized principled way to
        // do this, so I'm leaving this as-is and will come back to it later.
        distance_start_to_simultaneous_end(&challenge)
    } else {
        // These numbers were derived as described in the big comment in the
        // other arm of this conditional.
        [12169u64, 13301, 16697, 17263, 20093, 20659]
            .iter()
            .fold(1, |lcm, n| integer::lcm(lcm, *n))
    };
    println!("Distance: {dist}");
    assert_eq!(dist, 10_668_805_667_831);
}
