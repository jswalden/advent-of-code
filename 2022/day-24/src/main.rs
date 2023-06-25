use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::convert::From;
use std::ops::{BitAnd, BitOr, BitOrAssign};

#[repr(u8)]
#[derive(Copy, Clone)]
enum Wind {
    Up = 0b0001,
    Down = 0b0010,
    Left = 0b0100,
    Right = 0b1000,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct WindSet(u8);

impl From<Wind> for WindSet {
    fn from(value: Wind) -> Self {
        WindSet(value as u8)
    }
}

impl BitAnd<Wind> for WindSet {
    type Output = bool;

    fn bitand(self, rhs: Wind) -> Self::Output {
        self.0 & rhs as u8 != 0
    }
}

impl BitOr<Wind> for WindSet {
    type Output = WindSet;

    fn bitor(self, rhs: Wind) -> Self::Output {
        WindSet(self.0 | rhs as u8)
    }
}

impl BitOrAssign<Wind> for WindSet {
    fn bitor_assign(&mut self, rhs: Wind) {
        *self = WindSet(self.0 | rhs as u8);
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Square {
    Empty,
    Wall,
    Winds(WindSet),
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct WindState(Vec<Square>);

fn wrapping_add(coord: usize, amt: isize, limit: usize) -> usize {
    if coord == 0 && amt < 0 {
        return limit.checked_add_signed(amt).unwrap();
    }

    if coord == limit - 1 && amt > 0 {
        return amt as usize - 1;
    }

    coord
        .checked_add_signed(amt)
        .expect("amt must be smaller magnitude than side size")
}

struct WindSimulator {
    computed_winds: Vec<WindState>,
    start: (usize, usize),
    end: (usize, usize),
    width: usize,
    height: usize,
}

impl WindSimulator {
    fn new(s: &'static str) -> WindSimulator {
        let mut liter = s.lines();

        let is_wall_with_opening_at = |s: &str, opening_at| {
            s.chars()
                .enumerate()
                .all(|(i, c)| c == '#' || i == opening_at)
        };

        let mut table = vec![];
        let (width, mut height);
        {
            let mut add_line = |line: &'static str| {
                for c in line.chars() {
                    table.push(match c {
                        '#' => Square::Wall,
                        '.' => Square::Empty,
                        '>' => Square::Winds(Wind::Right.into()),
                        '<' => Square::Winds(Wind::Left.into()),
                        '^' => Square::Winds(Wind::Up.into()),
                        'v' => Square::Winds(Wind::Down.into()),
                        c => panic!("unexpected square: {c:?}"),
                    });
                }
            };

            let first = liter.next().expect("first line");
            width = first.len();
            height = 1;
            assert!(
                is_wall_with_opening_at(first, 1),
                "first line is a wall with one opening"
            );
            add_line(first);

            while let Some(line) = liter.next() {
                height += 1;
                add_line(line);
            }
        }

        let sim = WindSimulator {
            computed_winds: vec![WindState(table)],
            start: (1, 0),
            end: (width - 2, height - 1),
            width,
            height,
        };

        assert!(sim.square_at(&sim.computed_winds[0], sim.start) == Square::Empty);
        assert!(sim.square_at(&sim.computed_winds[0], sim.end) == Square::Empty);

        sim
    }

    fn start(&self) -> SimulatorState {
        SimulatorState {
            location: self.start,
            time: 0,
            leg: 0,
        }
    }

    fn find_empty(
        &self,
        winds: &mut WindState,
        (mut x, mut y): (usize, usize),
        w: Wind,
    ) -> (usize, usize) {
        let (delta_x, delta_y) = match w {
            Wind::Up => (0, -1),
            Wind::Down => (0, 1),
            Wind::Left => (-1, 0),
            Wind::Right => (1, 0),
        };

        loop {
            let cand_x = wrapping_add(x, delta_x, self.width);
            let cand_y = wrapping_add(y, delta_y, self.height);

            match self.square_at(winds, (cand_x, cand_y)) {
                Square::Wall => (x, y) = (cand_x, cand_y),
                _ => return (cand_x, cand_y),
            }
        }
    }

    fn ensure_winds_at_time(&mut self, t: usize) {
        while self.computed_winds.len() <= t {
            let last_winds = self.computed_winds.last().expect("t=0 always present");

            let mut winds = WindState(
                last_winds
                    .0
                    .iter()
                    .map(|s| match *s {
                        Square::Wall => Square::Wall,
                        _ => Square::Empty,
                    })
                    .collect(),
            );

            for x in 0..self.width {
                for y in 0..self.height {
                    match self.square_at(last_winds, (x, y)) {
                        Square::Wall => {
                            assert!(self.square_at(&winds, (x, y)) == Square::Wall);
                        }
                        Square::Empty => continue,
                        Square::Winds(wind_bits) => {
                            if wind_bits & Wind::Up {
                                let (x, y) = self.find_empty(&mut winds, (x, y), Wind::Up);
                                self.add_wind_at(&mut winds, (x, y), Wind::Up);
                            }
                            if wind_bits & Wind::Down {
                                let (x, y) = self.find_empty(&mut winds, (x, y), Wind::Down);
                                self.add_wind_at(&mut winds, (x, y), Wind::Down);
                            }
                            if wind_bits & Wind::Right {
                                let (x, y) = self.find_empty(&mut winds, (x, y), Wind::Right);
                                self.add_wind_at(&mut winds, (x, y), Wind::Right);
                            }
                            if wind_bits & Wind::Left {
                                let (x, y) = self.find_empty(&mut winds, (x, y), Wind::Left);
                                self.add_wind_at(&mut winds, (x, y), Wind::Left);
                            }
                        }
                    }
                }
            }

            self.computed_winds.push(winds);
        }
    }

    fn winds_at_time(&self, t: usize) -> &WindState {
        assert!(t < self.computed_winds.len());
        &self.computed_winds[t]
    }

    fn square_at(&self, winds: &WindState, (x, y): (usize, usize)) -> Square {
        assert!(x < self.width);
        assert!(y < self.height);
        winds.0[x + y * self.width]
    }

    fn add_wind_at(&self, winds: &mut WindState, (x, y): (usize, usize), w: Wind) {
        assert!(x < self.width);
        assert!(y < self.height);

        let square = &mut winds.0[x + y * self.width];
        match square {
            Square::Empty => *square = Square::Winds(w.into()),
            Square::Wall => {}
            Square::Winds(winds) => *winds |= w,
        }
    }

    fn priority_to(&self, state: &SimulatorState, end: (usize, usize)) -> Reverse<(usize, usize)> {
        let (x, y) = state.location;
        let (end_x, end_y) = end;
        let dist_to_end = end_x.abs_diff(x) + end_y.abs_diff(y);

        Reverse((state.time, dist_to_end))
    }

    fn leg_target(&self, leg: usize) -> (usize, usize) {
        if leg & 1 == 0 {
            self.end
        } else {
            self.start
        }
    }

    fn priority_with_trip_back(&self, state: &SimulatorState) -> Reverse<(usize, usize)> {
        let (x, y) = state.location;
        let (end_x, end_y) = self.leg_target(state.leg);

        let dist_to_leg_target = end_x.abs_diff(x) + end_y.abs_diff(y);

        let additional_trips_dist = (2 - state.leg)
            * (self.end.0.abs_diff(self.start.0) + self.end.1.abs_diff(self.start.1));

        let min_remaining_dist = dist_to_leg_target + additional_trips_dist;

        Reverse((state.time, min_remaining_dist))
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct SimulatorState {
    location: (usize, usize),
    time: usize,
    leg: usize,
}

fn find_shortest_time_to_end(sim: &mut WindSimulator) -> usize {
    let initial_state = sim.start();
    let initial_priority = sim.priority_to(&initial_state, sim.end);

    let mut frontier = PriorityQueue::new();
    frontier.push(initial_state, initial_priority);

    while let Some((
        SimulatorState {
            location,
            time,
            leg,
        },
        _,
    )) = frontier.pop()
    {
        assert!(sim.square_at(sim.winds_at_time(time), location) == Square::Empty);
        if location == sim.leg_target(leg) {
            return time;
        }

        let time = time + 1;
        sim.ensure_winds_at_time(time);

        let next_winds = sim.winds_at_time(time);

        macro_rules! try_location {
            ($x:expr, $y:expr) => {
                let location = ($x, $y);

                if sim.square_at(next_winds, location) == Square::Empty {
                    let state = SimulatorState {
                        location,
                        time,
                        leg,
                    };
                    let priority = sim.priority_to(&state, sim.end);
                    frontier.push(state, priority);
                }
            };
        }

        let (x, y) = location;

        if y + 1 < sim.height {
            try_location!(x, y + 1); // down
        }
        if x + 1 < sim.width {
            try_location!(x + 1, y); // right
        }
        if x > 0 {
            try_location!(x - 1, y); // left
        }
        if y > 0 {
            try_location!(x, y - 1); // up
        }
        try_location!(x, y); // sit pat
    }

    panic!("no valid path to end");
}

fn find_shortest_time_with_extra_roundtrip(sim: &mut WindSimulator) -> usize {
    let initial_state = sim.start();
    let initial_priority = sim.priority_with_trip_back(&initial_state);

    let mut frontier = PriorityQueue::new();
    frontier.push(initial_state, initial_priority);

    while let Some((
        SimulatorState {
            location,
            time,
            leg,
        },
        _,
    )) = frontier.pop()
    {
        assert!(sim.square_at(sim.winds_at_time(time), location) == Square::Empty);

        let leg_target = sim.leg_target(leg);

        let leg = if location == leg_target {
            if leg == 2 {
                return time;
            }

            leg + 1
        } else {
            leg
        };

        let time = time + 1;
        sim.ensure_winds_at_time(time);

        let next_winds = sim.winds_at_time(time);

        macro_rules! try_location {
            ($x:expr, $y:expr) => {
                let location = ($x, $y);

                if sim.square_at(next_winds, location) == Square::Empty {
                    let state = SimulatorState {
                        location,
                        time,
                        leg,
                    };
                    let priority = sim.priority_with_trip_back(&state);
                    frontier.push(state, priority);
                }
            };
        }

        let (x, y) = location;

        try_location!(x, y); // sit pat
        if y + 1 < sim.height {
            try_location!(x, y + 1); // down
        }
        if x + 1 < sim.width {
            try_location!(x + 1, y); // right
        }
        if x > 0 {
            try_location!(x - 1, y); // left
        }
        if y > 0 {
            try_location!(x, y - 1); // up
        }
    }

    panic!("no valid trip with extra roundtrip found");
}

#[test]
fn complex_example() {
    let input = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    let mut sim = WindSimulator::new(input);

    // Part 1.
    let t = find_shortest_time_to_end(&mut sim);
    println!("Part 1 time: {t}");
    assert_eq!(t, 18);

    // Part 2.
    let t = find_shortest_time_with_extra_roundtrip(&mut sim);
    println!("Part 2 time: {t} (should be 54)");
    assert_eq!(t, 54);
}

fn main() {
    let input = include_str!("../input");

    let mut sim = WindSimulator::new(input);

    // Part 1.
    let t = find_shortest_time_to_end(&mut sim);
    println!("Part 1 time: {t}");
    assert_eq!(t, 260);

    // Part 2.
    let t = find_shortest_time_with_extra_roundtrip(&mut sim);
    println!("Part 2 time: {t}");
    assert_eq!(t, 747);
}
