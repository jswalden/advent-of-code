use std::collections::{hash_map::Entry, HashMap};

#[derive(Copy, Clone)]
enum Push {
    Left,
    Right,
}

fn to_jet_pattern(input: &str) -> Vec<Push> {
    input
        .chars()
        .map(|c| match c {
            '>' => Push::Right,
            '<' => Push::Left,
            c => panic!("bad push: {c:?}"),
        })
        .collect::<Vec<_>>()
}

type RockInfo = (&'static [u8], usize, usize);

/// Array of rock-tuples:
///
/// * The first element of each tuple is the rock, encoded as its layers fro
///    bottom to top, smashed against the right wall.
/// * The second element is the width of the rock at its widest.
/// * The third element of each tuple is the amount the rock should be
///   left-shifted to place it in starting position horizontally.
const ROCKS: [RockInfo; 5] = [
    (&[0b1111], 4, 1),
    (&[0b010, 0b111, 0b010], 3, 2),
    (&[0b111, 0b001, 0b001], 3, 2),
    (&[0b1, 0b1, 0b1, 0b1], 1, 4),
    (&[0b11, 0b11], 2, 3),
];

const CHAMBER_WIDTH: usize = 7;
const EMPTY_SPACE_HEIGHT: usize = 3;

struct Chamber {
    layers: Vec<u8>,
    tower_height: u64,
    column_heights: [usize; CHAMBER_WIDTH],
}

impl Chamber {
    fn new() -> Chamber {
        Chamber {
            layers: vec![0b000_0000; EMPTY_SPACE_HEIGHT],
            tower_height: 0,
            column_heights: [0; CHAMBER_WIDTH],
        }
    }

    fn tower_height(&self) -> u64 {
        let chamber_layers = &self.layers;
        assert!(
            (&chamber_layers[chamber_layers.len() - EMPTY_SPACE_HEIGHT..])
                .iter()
                .all(|layer| *layer == 0),
            "must have expected empty space above"
        );
        assert!(
            chamber_layers.len() == EMPTY_SPACE_HEIGHT
                || chamber_layers[chamber_layers.len() - EMPTY_SPACE_HEIGHT - 1] != 0,
            "must have nonempty layer beneath them"
        );

        self.tower_height
    }
}

fn rock_overlaps_chamber(
    rock: &[u8],
    rock_bottom_idx: usize,
    rock_offset: usize,
    chamber: &Chamber,
) -> bool {
    let chamber_layers = &chamber.layers;
    rock.iter()
        .zip(&chamber_layers[rock_bottom_idx..])
        .any(|(rock_layer, chamber_layer)| chamber_layer & (rock_layer << rock_offset) != 0)
}

const DEBUG: bool = false;

#[allow(dead_code)]
fn dump_chamber(desc: &str, i: usize, chamber: &Chamber) {
    if !DEBUG || i >= 5 {
        return;
    }

    let chamber = &chamber.layers;

    println!("{desc}");

    let mut out = vec![];

    {
        let mut floor = String::from("+");
        floor += "-".repeat(CHAMBER_WIDTH as usize).as_str();
        floor.push('+');
        out.push(floor);
    }

    for layer in chamber {
        let mut line = String::from("|");

        let mut b = (1 as u8) << (CHAMBER_WIDTH - 1);
        while b > 0 {
            line.push(if b & *layer != 0 { '#' } else { '.' });
            b >>= 1;
        }

        line.push('|');
        out.push(line);
    }

    out.reverse();
    println!("{}\n", out.join("\n"));
}

#[allow(dead_code)]
fn dump_chamber_and_falling_rock(
    desc: &str,
    i: usize,
    chamber: &Chamber,
    rock: &[u8],
    rock_bottom_idx: usize,
    rock_offset: usize,
) {
    if !DEBUG || i >= 5 {
        return;
    }

    let chamber = &chamber.layers;

    println!("{desc}");

    let mut out = vec![];

    {
        let mut floor = String::from("+");
        floor += "-".repeat(CHAMBER_WIDTH as usize).as_str();
        floor.push('+');
        out.push(floor);
    }

    for (i, layer) in chamber.iter().enumerate() {
        let mut line = String::from("|");

        let rock_contrib = if rock_bottom_idx <= i && i < rock_bottom_idx + rock.len() {
            rock[i - rock_bottom_idx] << rock_offset
        } else {
            0
        };

        let layer = *layer | rock_contrib;

        let mut b = (1 as u8) << (CHAMBER_WIDTH - 1);
        while b > 0 {
            line.push(if b & layer != 0 { '#' } else { '.' });
            b >>= 1;
        }

        line.push('|');
        out.push(line);
    }

    out.reverse();
    println!("{}\n", out.join("\n"));
}

fn run(
    chamber: &mut Chamber,
    jet_pattern: &Vec<Push>,
    num_rocks: usize,
    (mut rock_idx, mut jet_idx): (usize, usize),
) -> (usize, usize) {
    for i in 0..num_rocks {
        let (rock, rock_width, rock_starting_offset) = ROCKS[rock_idx];
        rock_idx = (rock_idx + 1) % ROCKS.len();

        let mut rock_bottom_idx = chamber.layers.len();
        let mut rock_offset = rock_starting_offset;

        loop {
            let jet = jet_pattern[jet_idx];
            jet_idx = (jet_idx + 1) % jet_pattern.len();

            dump_chamber_and_falling_rock(
                "before pushing",
                i,
                chamber,
                rock,
                rock_bottom_idx,
                rock_offset,
            );

            // Push rock.
            let cand_rock_offset = match jet {
                Push::Left => rock_offset + 1,
                Push::Right => rock_offset.saturating_sub(1),
            };
            if cand_rock_offset != rock_offset {
                if cand_rock_offset + rock_width <= CHAMBER_WIDTH {
                    if !rock_overlaps_chamber(rock, rock_bottom_idx, cand_rock_offset, chamber) {
                        rock_offset = cand_rock_offset;
                    }
                }
            }

            dump_chamber_and_falling_rock(
                "after pushing",
                i,
                chamber,
                rock,
                rock_bottom_idx,
                rock_offset,
            );

            // Rock falls.
            if rock_bottom_idx == 0 {
                break;
            }

            if rock_overlaps_chamber(rock, rock_bottom_idx - 1, rock_offset, chamber) {
                break;
            }

            rock_bottom_idx -= 1;
        }

        // Apply rock from bottom up, then pad with empty layers.
        for (i, rock_layer) in rock.iter().enumerate() {
            let chamber_layer_idx = rock_bottom_idx + i;
            if chamber_layer_idx == chamber.layers.len() {
                chamber.layers.push(0);
                chamber.tower_height += 1;
            }

            let chamber_layer = &mut chamber.layers[chamber_layer_idx];
            let rock = *rock_layer << rock_offset;
            *chamber_layer |= rock;

            let mut rock_bits = rock;
            let mut col = 0;
            while rock_bits > 0 {
                if rock_bits & 1 != 0 {
                    chamber.column_heights[col] =
                        chamber.column_heights[col].max(chamber_layer_idx + 1);
                }

                rock_bits >>= 1;
                col += 1;
            }
        }

        let empty_layers_count = chamber
            .layers
            .iter()
            .rev()
            .take(EMPTY_SPACE_HEIGHT)
            .take_while(|layer| **layer == 0)
            .count();
        (empty_layers_count..EMPTY_SPACE_HEIGHT).for_each(|_| {
            chamber.layers.push(0b000_0000);
            chamber.tower_height += 1
        });

        dump_chamber("after placement:", i, chamber);
    }

    (rock_idx, jet_idx)
}

fn part1(jet_pattern: &Vec<Push>, expected: u64) {
    let mut chamber = Chamber::new();

    const NUM_ROCKS: usize = 2022;

    let (rock_idx, jet_idx) = (0, 0);
    let (next_rock_idx, _next_jet_idx) =
        run(&mut chamber, &jet_pattern, NUM_ROCKS, (rock_idx, jet_idx));
    assert_eq!(next_rock_idx, NUM_ROCKS % ROCKS.len());

    let tower_height = chamber.tower_height();
    println!("Tower height: {tower_height}");
    assert_eq!(tower_height, expected);
}

fn part2(jet_pattern: &Vec<Push>, expected_tower_height: u64) {
    const GAZILLION_ROCKS_DROPPED_COUNT: u64 = 1_000_000_000_000;

    let mut chamber = Chamber::new();

    let (mut rock_idx, mut jet_idx) = (0, 0);
    let mut rocks_dropped = 0u64;

    #[derive(PartialEq, Eq, Hash)]
    struct Key {
        rock_idx: usize,
        jet_idx: usize,
    }
    struct Value {
        rocks_dropped_delta: u64,
        rocks_dropped: u64,
        tower_height: u64,
    }

    let mut seen = HashMap::<Key, Value>::new();

    let mut found_cycle = false;
    loop {
        (rock_idx, jet_idx) = run(&mut chamber, jet_pattern, 1, (rock_idx, jet_idx));
        rocks_dropped += 1;

        if !found_cycle {
            match seen.entry(Key { rock_idx, jet_idx }) {
                Entry::Occupied(mut oe) => {
                    let val = oe.get_mut();

                    let rocks_dropped_delta = rocks_dropped - val.rocks_dropped;
                    if rocks_dropped_delta == val.rocks_dropped_delta {
                        found_cycle = true;

                        let cycle_length = rocks_dropped_delta;
                        let cycle_height_increase = chamber.tower_height() - val.tower_height;

                        let cycle_count =
                            (GAZILLION_ROCKS_DROPPED_COUNT - rocks_dropped) / cycle_length;

                        rocks_dropped += cycle_count * rocks_dropped_delta;
                        chamber.tower_height += cycle_count * cycle_height_increase;
                    }

                    val.rocks_dropped_delta = rocks_dropped_delta;
                    val.rocks_dropped = rocks_dropped;
                    val.tower_height = chamber.tower_height();
                }
                Entry::Vacant(ve) => {
                    ve.insert(Value {
                        rocks_dropped_delta: 0,
                        rocks_dropped,
                        tower_height: chamber.tower_height(),
                    });
                }
            }
        }

        if rocks_dropped == GAZILLION_ROCKS_DROPPED_COUNT {
            break;
        }
    }

    assert!(rocks_dropped == GAZILLION_ROCKS_DROPPED_COUNT);

    let total_tower_height = chamber.tower_height();
    println!("Total tower height: {total_tower_height}");
    assert_eq!(total_tower_height, expected_tower_height);
}

#[test]
fn test_example() {
    static INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    let jet_pattern = to_jet_pattern(INPUT);

    part1(&jet_pattern, 3068);
    part2(&jet_pattern, 1_514_285_714_288);
}

fn main() {
    static INPUT: &str = include_str!("../input");
    let jet_pattern = to_jet_pattern(INPUT.trim());

    part1(&jet_pattern, 3171);
    part2(&jet_pattern, 1_586_627_906_921);
}
