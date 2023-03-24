use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

type Coord = i16;
type Location = (Coord, Coord);
type ElfSet = HashSet<Location>;

struct Nearby {
    occupied: [bool; 8],
}

impl Nearby {
    const NORTH: ([usize; 3], (Coord, Coord)) = ([0, 1, 2], (0, -1));
    const SOUTH: ([usize; 3], (Coord, Coord)) = ([5, 6, 7], (0, 1));
    const WEST: ([usize; 3], (Coord, Coord)) = ([0, 3, 5], (-1, 0));
    const EAST: ([usize; 3], (Coord, Coord)) = ([2, 4, 7], (1, 0));

    fn new(elves: &ElfSet, (x, y): Location) -> Nearby {
        Nearby {
            occupied: [
                elves.contains(&(x - 1, y - 1)),
                elves.contains(&(x, y - 1)),
                elves.contains(&(x + 1, y - 1)),
                elves.contains(&(x - 1, y)),
                elves.contains(&(x + 1, y)),
                elves.contains(&(x - 1, y + 1)),
                elves.contains(&(x, y + 1)),
                elves.contains(&(x + 1, y + 1)),
            ],
        }
    }

    fn isolated(&self) -> bool {
        self.occupied.iter().all(|o| !o)
    }
}

const DIRECTIONS_ARRAY: [&([usize; 3], (Coord, Coord)); 4] =
    [&Nearby::NORTH, &Nearby::SOUTH, &Nearby::WEST, &Nearby::EAST];

fn propose_move(
    proposed_locs: &mut HashMap<Location, Option<Location>>,
    unmoving_locs: &mut HashSet<Location>,
    loc: Location,
    nearby: &Nearby,
    locs: &[usize; 3],
    delta: &(Coord, Coord),
) -> bool {
    if nearby.occupied[locs[0]] || nearby.occupied[locs[1]] || nearby.occupied[locs[2]] {
        return false;
    }

    let new_loc = (loc.0 + delta.0, loc.1 + delta.1);
    match proposed_locs.entry(new_loc) {
        Entry::Occupied(mut oe) => {
            let prior = oe.get_mut();
            match *prior {
                Some(prior_loc) => {
                    unmoving_locs.insert(prior_loc);
                    *prior = None;
                }
                None => {}
            }
            unmoving_locs.insert(loc);
        }
        Entry::Vacant(ve) => {
            ve.insert(Some(loc));
        }
    }

    true
}

#[derive(Copy, Clone, Debug)]
struct Bounds {
    min_x: Coord,
    max_x: Coord,
    min_y: Coord,
    max_y: Coord,
}

impl Bounds {
    fn new(elves: &ElfSet) -> Bounds {
        let (mut min_x, mut max_x) = (Coord::MAX, Coord::MIN);
        let (mut min_y, mut max_y) = (Coord::MAX, Coord::MIN);
        for (x, y) in elves {
            min_x = min_x.min(*x);
            max_x = max_x.max(*x);
            min_y = min_y.min(*y);
            max_y = max_y.max(*y);
        }

        Bounds {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }
}

struct State {
    elves: ElfSet,
    elf_count: usize,
    starting_direction: usize,
    bounds: Bounds,
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = (self.bounds.max_x as i32 - self.bounds.min_x as i32) as usize + 1;
        let height = (self.bounds.max_y as i32 - self.bounds.min_y as i32) as usize + 1;
        let mut out = vec![vec!["."; width]; height];

        for (x, y) in &self.elves {
            let rel_x = *x as i32 - self.bounds.min_x as i32;
            let rel_y = *y as i32 - self.bounds.min_y as i32;
            out[rel_y as usize][rel_x as usize] = "#";
        }

        for line in out {
            writeln!(f, "{}", line.join(""))?;
        }

        Ok(())
    }
}

impl State {
    fn new(elves: ElfSet) -> State {
        let elf_count = elves.len();
        let bounds = Bounds::new(&elves);

        State {
            elves,
            elf_count,
            starting_direction: 0,
            bounds,
        }
    }

    fn one_round(&mut self) -> bool {
        // Proposed new locations for all elves.
        let mut proposed_locs = HashMap::<Location, Option<Location>>::new();

        // Elves that don't move.
        let mut unmoving_locs = ElfSet::new();

        'next_loc: for loc in self.elves.iter().copied() {
            let nearby = Nearby::new(&self.elves, loc);
            if !nearby.isolated() {
                for i in 0..DIRECTIONS_ARRAY.len() {
                    let i = (i + self.starting_direction) % DIRECTIONS_ARRAY.len();

                    let (locs, delta) = DIRECTIONS_ARRAY[i];
                    if propose_move(
                        &mut proposed_locs,
                        &mut unmoving_locs,
                        loc,
                        &nearby,
                        locs,
                        delta,
                    ) {
                        continue 'next_loc;
                    }
                }
            }

            unmoving_locs.insert(loc);
        }

        self.starting_direction = (self.starting_direction + 1) % DIRECTIONS_ARRAY.len();

        if unmoving_locs.len() == self.elves.len() {
            return false;
        }

        unmoving_locs.extend(
            proposed_locs
                .into_iter()
                .filter(|(_, maybe_original)| maybe_original.is_some())
                .map(|(loc, _)| loc),
        );

        assert_eq!(
            self.elves.len(),
            unmoving_locs.len(),
            "must not lose track of any elves"
        );
        self.bounds = Bounds::new(&unmoving_locs);
        self.elves = unmoving_locs;

        true
    }

    fn simulate_n_rounds(&mut self, rounds: u32) -> u64 {
        for _ in 0..rounds {
            self.one_round();
        }

        let (mut min_x, mut max_x) = (Coord::MAX, Coord::MIN);
        let (mut min_y, mut max_y) = (Coord::MAX, Coord::MIN);
        for (x, y) in &self.elves {
            min_x = min_x.min(*x);
            max_x = max_x.max(*x);
            min_y = min_y.min(*y);
            max_y = max_y.max(*y);
        }

        let (p1, p2) = ((min_x, min_y), (max_x, max_y));

        (p2.0 as i32 - p1.0 as i32 + 1) as u64 * (p2.1 as i32 - p1.1 as i32 + 1) as u64
            - self.elf_count as u64
    }

    fn simulate_until_no_movement(&mut self) -> u32 {
        let mut n = 1;
        loop {
            if !self.one_round() {
                break;
            }
            n += 1;
        }
        n
    }
}

fn parse_input(s: &'static str) -> State {
    let mut x;
    let mut y = 0;

    let mut elves = ElfSet::new();
    for line in s.lines() {
        x = 0;
        for c in line.chars() {
            match c {
                '#' => {
                    elves.insert((x, y));
                }
                '.' => { /* empty */ }
                c => panic!("unexpected symbol: {c:?}"),
            }
            x += 1;
        }

        y += 1;
    }

    State::new(elves)
}

const PART1_ROUNDS: u32 = 10;

#[test]
fn small_example() {
    static INPUT: &str = ".....
..##.
..#..
.....
..##.
.....";

    // Part 1.
    let mut state = parse_input(INPUT);
    let empty_squares = state.simulate_n_rounds(PART1_ROUNDS);
    println!("Part 1 empty squares: {empty_squares}");
    assert_eq!(empty_squares, 5 * 6 - 5);
}

#[test]
fn example() {
    static INPUT: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    // Part 1.
    let mut state = parse_input(INPUT);
    let empty_squares = state.simulate_n_rounds(PART1_ROUNDS);
    println!("Part 1 empty squares: {empty_squares}");
    assert_eq!(empty_squares, 110);

    // Part 2.
    let added_rounds = state.simulate_until_no_movement();
    let rounds_til_no_motion = PART1_ROUNDS + added_rounds;
    println!("Part 2: {rounds_til_no_motion} rounds to no movement");
    assert_eq!(rounds_til_no_motion, 20);
}

fn main() {
    static INPUT: &str = include_str!("../input");

    // Part 1.
    let mut state = parse_input(INPUT);
    let empty_squares = state.simulate_n_rounds(PART1_ROUNDS);
    println!("Part 1 empty squares: {empty_squares}");
    assert_eq!(empty_squares, 3766);

    // Part 2.
    let added_rounds = state.simulate_until_no_movement();
    let rounds_til_no_motion = PART1_ROUNDS + added_rounds;
    println!("Part 2: {rounds_til_no_motion} rounds to no movement");
    assert_eq!(rounds_til_no_motion, 954);
}
