use priority_queue::PriorityQueue;
use std::cmp::Reverse;

const fn elevation_value(c: char) -> u8 {
    c as u8 - 'a' as u8
}

type Loc = (usize, usize);

struct Grid {
    array: Vec<u8>,
    width: usize,
    height: usize,
    start: Loc,
    end: Loc,
}

impl Grid {
    fn assert_in_range(&self, loc: Loc) {
        assert!(loc.0 < self.width, "bad x");
        assert!(loc.1 < self.height, "bad y");
    }

    fn height(&self, loc: Loc) -> u8 {
        self.assert_in_range(loc);

        let (x, y) = loc;
        self.array[y * self.width + x]
    }

    fn adjacent_locations<'a>(&'a self, loc: Loc) -> AdjacentLocations<'a> {
        self.assert_in_range(loc);

        AdjacentLocations::new(self, loc)
    }

    fn adjacent_reverse_locations<'a>(&'a self, loc: Loc) -> AdjacentReverseLocations<'a> {
        self.assert_in_range(loc);

        AdjacentReverseLocations::new(self, loc)
    }

    fn is_end(&self, loc: Loc) -> bool {
        self.assert_in_range(loc);

        loc == self.end
    }

    fn is_any_start(&self, loc: Loc) -> bool {
        self.assert_in_range(loc);

        self.height(loc) == 0
    }
}

fn parse_input(s: &str) -> Grid {
    let mut array = vec![];
    let mut width = 0;

    let mut start = None;
    let mut end = None;

    for (y, line) in s.lines().enumerate() {
        if width == 0 {
            width = line.len();
        } else {
            assert_eq!(line.len(), width);
        }

        for (x, mut c) in line.chars().enumerate() {
            if c == 'S' {
                start = Some((x, y));
                c = 'a';
            } else if c == 'E' {
                end = Some((x, y));
                c = 'z';
            }

            array.push(elevation_value(c));
        }
    }

    assert!(array.len() % width == 0);

    let height = array.len() / width;
    let start = start.expect("starting location");
    let end = end.expect("ending location");

    Grid {
        array,
        width,
        height,
        start,
        end,
    }
}

struct AdjacentLocations<'a> {
    grid: &'a Grid,
    loc: (usize, usize),
    phase: u8,
}

impl<'a> AdjacentLocations<'a> {
    fn new(grid: &'a Grid, loc: (usize, usize)) -> AdjacentLocations<'a> {
        assert!(loc.0 < grid.width);
        assert!(loc.1 < grid.height);

        AdjacentLocations {
            grid,
            loc,
            phase: 0,
        }
    }
}

impl<'a> Iterator for AdjacentLocations<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.phase >= 4 {
            return None;
        }

        let (x, y) = self.loc;

        let max_height = self.grid.height((x, y)) + 1;

        // Upward.
        if self.phase == 0 {
            self.phase += 1;

            if y > 0 {
                let loc = (x, y - 1);
                if self.grid.height(loc) <= max_height {
                    return Some(loc);
                }
            }
        }

        // Rightward.
        if self.phase == 1 {
            self.phase += 1;

            if x < self.grid.width - 1 {
                let loc = (x + 1, y);
                if self.grid.height(loc) <= max_height {
                    return Some(loc);
                }
            }
        }

        // Downward.
        if self.phase == 2 {
            self.phase += 1;

            if y < self.grid.height - 1 {
                let loc = (x, y + 1);
                if self.grid.height(loc) <= max_height {
                    return Some(loc);
                }
            }
        }

        // Leftward.
        if self.phase == 3 {
            self.phase += 1;

            if x > 0 {
                let loc = (x - 1, y);
                if self.grid.height(loc) <= max_height {
                    return Some(loc);
                }
            }
        }

        // Done.
        None
    }
}

struct AdjacentReverseLocations<'a> {
    grid: &'a Grid,
    loc: (usize, usize),
    phase: u8,
}

impl<'a> AdjacentReverseLocations<'a> {
    fn new(grid: &'a Grid, loc: (usize, usize)) -> AdjacentReverseLocations<'a> {
        assert!(loc.0 < grid.width);
        assert!(loc.1 < grid.height);

        AdjacentReverseLocations {
            grid,
            loc,
            phase: 0,
        }
    }
}

impl<'a> Iterator for AdjacentReverseLocations<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.phase >= 4 {
            return None;
        }

        let (x, y) = self.loc;

        let min_height = self.grid.height((x, y)) - 1;

        // Upward.
        if self.phase == 0 {
            self.phase += 1;

            if y > 0 {
                let loc = (x, y - 1);
                if self.grid.height(loc) >= min_height {
                    return Some(loc);
                }
            }
        }

        // Rightward.
        if self.phase == 1 {
            self.phase += 1;

            if x < self.grid.width - 1 {
                let loc = (x + 1, y);
                if self.grid.height(loc) >= min_height {
                    return Some(loc);
                }
            }
        }

        // Downward.
        if self.phase == 2 {
            self.phase += 1;

            if y < self.grid.height - 1 {
                let loc = (x, y + 1);
                if self.grid.height(loc) >= min_height {
                    return Some(loc);
                }
            }
        }

        // Leftward.
        if self.phase == 3 {
            self.phase += 1;

            if x > 0 {
                let loc = (x - 1, y);
                if self.grid.height(loc) >= min_height {
                    return Some(loc);
                }
            }
        }

        // Done.
        None
    }
}

#[test]
fn test_elevation_value() {
    assert_eq!(elevation_value('a'), 0);
    assert_eq!(elevation_value('z'), 25);
}

#[derive(Copy, Clone)]
struct Square {
    steps: usize,
    prev_loc: Loc,
}

struct PathTracking<'a> {
    grid: &'a Grid,
    locations: Vec<Square>,
}

impl<'a> PathTracking<'a> {
    fn new(grid: &Grid) -> PathTracking {
        let steps = usize::MAX;
        let prev_loc = (usize::MAX, usize::MAX);
        PathTracking {
            grid,
            locations: vec![Square { steps, prev_loc }; grid.height * grid.width],
        }
    }

    fn location(&self, loc: Loc) -> &Square {
        assert!(loc.0 < self.grid.width);
        assert!(loc.1 < self.grid.height);
        &self.locations[loc.1 * self.grid.width + loc.0]
    }

    fn location_mut(&mut self, loc: Loc) -> &mut Square {
        assert!(loc.0 < self.grid.width);
        assert!(loc.1 < self.grid.height);
        &mut self.locations[loc.1 * self.grid.width + loc.0]
    }

    fn best_steps(&self, loc: Loc) -> usize {
        self.location(loc).steps
    }

    fn update_best_path(&mut self, loc: Loc, prev_loc: Loc, steps: usize) {
        let info = self.location_mut(loc);
        info.prev_loc = prev_loc;
        info.steps = steps;
    }
}

fn find_shortest_path_length(grid: &Grid) {
    let mut path_tracking = PathTracking::new(&grid);

    let mut frontier = PriorityQueue::new();
    frontier.push(grid.start, Reverse(0usize));

    while let Some((loc, steps)) = frontier.pop() {
        let steps = steps.0;
        if grid.is_end(loc) {
            println!("Found end at ({}, {}) after {} steps", loc.0, loc.1, steps);
            break;
        }

        let new_steps = steps + 1;
        for adj_loc in grid.adjacent_locations(loc) {
            if path_tracking.best_steps(adj_loc) > new_steps {
                path_tracking.update_best_path(adj_loc, loc, new_steps);
                frontier.push_decrease(adj_loc, Reverse(new_steps));
            }
        }
    }
}

fn find_shortest_path_length_any_start(grid: &Grid) {
    let mut path_tracking = PathTracking::new(&grid);

    let mut frontier = PriorityQueue::new();
    frontier.push(grid.end, Reverse(0usize));

    while let Some((loc, steps)) = frontier.pop() {
        let steps = steps.0;
        if grid.is_any_start(loc) {
            println!(
                "Found a start at ({}, {}) after {} steps",
                loc.0, loc.1, steps
            );
            break;
        }

        let new_steps = steps + 1;
        for adj_loc in grid.adjacent_reverse_locations(loc) {
            if path_tracking.best_steps(adj_loc) > new_steps {
                path_tracking.update_best_path(adj_loc, loc, new_steps);
                frontier.push_decrease(adj_loc, Reverse(new_steps));
            }
        }
    }
}

fn main() {
    let grid = parse_input(include_str!("../input"));

    find_shortest_path_length(&grid);

    find_shortest_path_length_any_start(&grid);
}
