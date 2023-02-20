#[derive(Copy, Clone)]
enum Push {
    Left,
    Right,
}

type RockInfo = (&'static [u8], i32, i32);

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

fn to_jet_pattern(input: &str) -> Vec<Push> {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '>' => Push::Right,
            '<' => Push::Left,
            c => panic!("bad push: {c}"),
        })
        .collect::<Vec<_>>()
}

fn rock_overlaps_chamber(
    rock: &[u8],
    rock_bottom_idx: usize,
    rock_offset: i32,
    chamber_layers: &Vec<u8>,
) -> bool {
    rock.iter()
        .zip(&chamber_layers[rock_bottom_idx..])
        .any(|(rock_layer, chamber_layer)| chamber_layer & (rock_layer << rock_offset) != 0)
}

const CHAMBER_WIDTH: i32 = 7;
const EMPTY_SPACE_HEIGHT: usize = 3;

const DEBUG: bool = false;

#[allow(dead_code)]
fn dump_chamber(chamber: &Vec<u8>) {
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
    chamber: &Vec<u8>,
    rock: &[u8],
    rock_bottom_idx: usize,
    rock_offset: i32,
) {
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

fn run(chamber_layers: &mut Vec<u8>, jet_pattern: &Vec<Push>, num_rocks: usize) {
    let mut rock_idx = 0;

    let mut jet_idx = 0;

    for _i in 0..num_rocks {
        let (rock, rock_width, rock_starting_offset) = ROCKS[rock_idx];
        rock_idx = (rock_idx + 1) % ROCKS.len();

        let mut rock_bottom_idx = chamber_layers.len();
        let mut rock_offset = rock_starting_offset;

        loop {
            let jet = jet_pattern[jet_idx];
            jet_idx = (jet_idx + 1) % jet_pattern.len();

            if _i < 5 && DEBUG {
                println!("before pushing");
                dump_chamber_and_falling_rock(&chamber_layers, rock, rock_bottom_idx, rock_offset);
            }

            // Push rock.
            let cand_rock_offset = match jet {
                Push::Left => rock_offset + 1,
                Push::Right => rock_offset - 1,
            };
            if 0 <= cand_rock_offset && cand_rock_offset + rock_width <= CHAMBER_WIDTH {
                if !rock_overlaps_chamber(rock, rock_bottom_idx, cand_rock_offset, &chamber_layers)
                {
                    rock_offset = cand_rock_offset;
                }
            }

            if _i < 5 && DEBUG {
                println!("after pushing");
                dump_chamber_and_falling_rock(&chamber_layers, rock, rock_bottom_idx, rock_offset);
            }

            // Rock falls.
            if rock_bottom_idx == 0 {
                break;
            }

            if rock_overlaps_chamber(rock, rock_bottom_idx - 1, rock_offset, &chamber_layers) {
                break;
            }

            rock_bottom_idx -= 1;
        }

        // Apply rock from bottom up, then pad with empty layers.
        for (i, rock_layer) in rock.iter().enumerate() {
            let chamber_layer_idx = rock_bottom_idx + i;
            if chamber_layer_idx == chamber_layers.len() {
                chamber_layers.push(0);
            }

            let chamber_layer = &mut chamber_layers[chamber_layer_idx];
            *chamber_layer |= *rock_layer << rock_offset;
        }

        let empty_layers_count = chamber_layers
            .iter()
            .rev()
            .take(EMPTY_SPACE_HEIGHT)
            .take_while(|layer| **layer == 0)
            .count();
        (empty_layers_count..EMPTY_SPACE_HEIGHT).for_each(|_| {
            chamber_layers.push(0);
        });

        if _i < 5 && DEBUG {
            println!("after placement:");
            dump_chamber(&chamber_layers);
        }
    }
}

fn part1(jet_pattern: &Vec<Push>, expected: usize) {
    // Chamber layers, encoded bottom to top.
    let mut chamber_layers: Vec<u8> = vec![0b0000000; EMPTY_SPACE_HEIGHT];

    run(&mut chamber_layers, &jet_pattern, 2022);

    let tower_height = chamber_layers.len()
        - chamber_layers
            .iter()
            .rev()
            .take_while(|chamber_layer| **chamber_layer == 0)
            .count();
    println!("Tower height: {tower_height}");
    assert_eq!(tower_height, expected);
}

fn part2(jet_pattern: &Vec<Push>) {}

#[test]
fn test_example() {
    static INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    let jet_pattern = to_jet_pattern(INPUT);

    part1(&jet_pattern, 3068);
    part2(&jet_pattern);
}

fn main() {
    static INPUT: &str = include_str!("../input");
    let jet_pattern = to_jet_pattern(INPUT);

    part1(&jet_pattern, 3171);
    part2(&jet_pattern);
}
