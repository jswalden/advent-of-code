use std::collections::HashSet;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

static DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

impl Direction {
    fn to_offset(&self) -> (isize, isize) {
        match *self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }

    fn all() -> impl Iterator<Item = Direction> {
        DIRECTIONS.iter().copied()
    }

    fn reverse(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }

    fn turn(&self, turn: Side) -> Direction {
        let i = *self as usize;

        DIRECTIONS[(i + match turn {
            Side::Right => 1,
            Side::Left => DIRECTIONS.len() - 1,
        }) % DIRECTIONS.len()]
    }
}

#[test]
fn directions_test() {
    assert_eq!(Direction::Up.turn(Side::Left), Direction::Left);
    assert_eq!(Direction::Up.turn(Side::Right), Direction::Right);

    assert_eq!(Direction::Right.turn(Side::Left), Direction::Up);
    assert_eq!(Direction::Right.turn(Side::Right), Direction::Down);

    assert_eq!(Direction::Down.turn(Side::Left), Direction::Right);
    assert_eq!(Direction::Down.turn(Side::Right), Direction::Left);

    assert_eq!(Direction::Left.turn(Side::Left), Direction::Down);
    assert_eq!(Direction::Left.turn(Side::Right), Direction::Up);
}

#[derive(Clone, Copy, Debug)]
enum Tile {
    Pipe([Direction; 2]),
    Ground,
    Start,
}

impl Tile {
    fn is_pipe(&self) -> bool {
        match self {
            Tile::Pipe(_) | Tile::Start => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Side {
    Left,
    Right,
}

struct Path<'a> {
    sketch: &'a Sketch,
    tiles: HashSet<(usize, usize)>,
    starting_loc: (usize, usize),
    next: (usize, usize),
    start_to_next: Direction,
    top_row: usize,
    inside_side: Side,
}

impl<'a> Path<'a> {
    fn new(
        sketch: &'a Sketch,
        start: (usize, usize),
        start_to_next: Direction,
        next: (usize, usize),
    ) -> Path {
        assert_eq!(
            start
                .0
                .overflowing_add_signed(start_to_next.to_offset().0)
                .0,
            next.0
        );
        assert_eq!(
            start
                .1
                .overflowing_add_signed(start_to_next.to_offset().1)
                .0,
            next.1
        );

        // The eventual top row contain at least tile-pair, so this initial
        // value is meaningless because it'll be ovewritten before the path is
        // complete.
        let inside_side = Side::Left;

        Path {
            sketch,
            tiles: HashSet::from([next]),
            starting_loc: start,
            next,
            start_to_next,
            top_row: start.0,
            inside_side,
        }
    }

    fn extend(&mut self, curr: (usize, usize), next: (usize, usize)) {
        self.tiles.insert(next);

        self.top_row = self.top_row.min(next.0);

        if curr.0 == next.0 && curr.0 == self.top_row {
            self.inside_side = if curr.1 < next.1 {
                Side::Right
            } else {
                Side::Left
            };
        }
    }

    fn greatest_distance_from_start(&self) -> usize {
        self.tiles.len() / 2
    }

    fn count_inside(&self) -> usize {
        let inside_side = self.inside_side;

        let mut prev = self.starting_loc;
        let mut prev_to_curr = self.start_to_next;
        let mut curr = self.next;

        let mut adjacent_on_inside = HashSet::new();
        while curr != self.starting_loc {
            let absolute_dir_inside = prev_to_curr.turn(inside_side);

            // Be sure to consider the insides of *both* tiles.  If only one
            // tile is considered, a deviously constructed input (the tests
            // within `inside_area_test_both_prev_and_curr` cover either
            // omission) can miss either adjacent tile in certain edge cases.
            if let Some(inside) = self.sketch.try_offset(prev, absolute_dir_inside) {
                if !self.tiles.contains(&inside) {
                    adjacent_on_inside.insert(inside);
                }
            }
            if let Some(inside) = self.sketch.try_offset(curr, absolute_dir_inside) {
                if !self.tiles.contains(&inside) {
                    adjacent_on_inside.insert(inside);
                }
            }

            let Some((curr_to_next, next)) = self.sketch.find_next(prev, prev_to_curr, curr, self, false) else {
                panic!("path-tracing failure")
            };

            prev = curr;
            prev_to_curr = curr_to_next;
            curr = next;
        }

        let mut insides = HashSet::new();

        let mut workset = adjacent_on_inside;
        while workset.len() > 0 {
            let mut next_workset = HashSet::new();
            for loc in workset.drain() {
                let newly_inserted = insides.insert(loc.clone());
                if !newly_inserted {
                    continue;
                };

                for dir in Direction::all() {
                    let Some(discovered_loc) = self.sketch.try_offset(loc, dir) else {
                        panic!("walking off edge?")
                    };
                    if self.tiles.contains(&discovered_loc) || insides.contains(&discovered_loc) {
                        continue;
                    }
                    next_workset.insert(discovered_loc);
                }
            }

            workset.extend(next_workset);
        }

        insides.len()
    }
}

struct Sketch {
    starting_loc: (usize, usize),
    grid: Vec<Tile>,
    grid_width: usize,
    grid_height: usize,
}

impl Sketch {
    fn new(diagram: &str) -> Sketch {
        let mut starting_loc = (0, 0);

        let mut grid = vec![];
        let mut grid_width = 0;
        let mut grid_height = 0;
        for (y, line) in diagram.trim().lines().enumerate() {
            grid_height = y + 1;
            let line_len = line.len();
            if grid_width == 0 {
                grid_width = line_len;
            } else {
                assert_eq!(grid_width, line_len);
            }

            grid.extend(line.chars().enumerate().map(|(x, c)| match c {
                '|' => Tile::Pipe([Direction::Up, Direction::Down]),
                '-' => Tile::Pipe([Direction::Left, Direction::Right]),
                'L' => Tile::Pipe([Direction::Up, Direction::Right]),
                'J' => Tile::Pipe([Direction::Up, Direction::Left]),
                '7' => Tile::Pipe([Direction::Down, Direction::Left]),
                'F' => Tile::Pipe([Direction::Down, Direction::Right]),
                '.' => Tile::Ground,
                'S' => {
                    starting_loc = (y, x);
                    Tile::Start
                }
                c => panic!("unexpected tile: {c:?}"),
            }));
        }

        Sketch {
            starting_loc,
            grid,
            grid_width: grid_width,
            grid_height,
        }
    }

    fn at(&self, (y, x): (usize, usize)) -> Tile {
        assert!(y < self.grid_height);
        assert!(x < self.grid_width);
        self.grid[y * self.grid_width + x]
    }

    fn try_offset(&self, loc: (usize, usize), dir: Direction) -> Option<(usize, usize)> {
        let offset = dir.to_offset();
        let y = loc
            .0
            .checked_add_signed(offset.0)
            .filter(|y| *y < self.grid_height);
        let x = loc
            .1
            .checked_add_signed(offset.1)
            .filter(|x| *x < self.grid_width);

        if let (Some(y), Some(x)) = (y, x) {
            Some((y, x))
        } else {
            None
        }
    }

    fn find_next(
        &self,
        prev: (usize, usize),
        prev_to_curr: Direction,
        curr: (usize, usize),
        path: &Path,
        unverified: bool,
    ) -> Option<(Direction, (usize, usize))> {
        assert!(self.at(prev).is_pipe());
        assert!(self.at(curr).is_pipe());

        let curr_tile = self.at(curr);
        let Tile::Pipe(curr_directions) = curr_tile else {
            panic!("current tile expected to be pipe: {curr_tile:?}");
        };
        let curr_to_prev = prev_to_curr.reverse();
        assert!(curr_directions.contains(&curr_to_prev));

        let next_dir = curr_directions
            .into_iter()
            .filter(|dir| *dir != curr_to_prev)
            .next()
            .expect("next_dir");

        self.try_offset(curr, next_dir).and_then(|cand_loc| {
            if unverified {
                if path.tiles.contains(&cand_loc) {
                    return None;
                }
                match self.at(cand_loc) {
                    Tile::Pipe(directions) => {
                        if !directions.contains(&next_dir.reverse()) {
                            return None;
                        }
                    }
                    Tile::Ground => return None,
                    Tile::Start => {}
                }
            }

            Some((next_dir, cand_loc))
        })
    }

    fn first_steps(&self) -> impl Iterator<Item = (Direction, (usize, usize))> {
        let loc = self.starting_loc;
        let mut results = vec![];

        let mut try_offset = |direction: Direction| {
            if let Some(cand) = self.try_offset(loc, direction) {
                match self.at(cand) {
                    Tile::Pipe(directions) => {
                        if directions.contains(&direction.reverse()) {
                            results.push((direction, cand));
                        }
                    }
                    Tile::Ground => {}
                    Tile::Start => panic!("start next to start?"),
                }
            }
        };

        try_offset(Direction::Up);
        try_offset(Direction::Right);
        try_offset(Direction::Down);
        try_offset(Direction::Left);

        results.into_iter()
    }

    fn compute_path(&self) -> Path {
        for (start_to_neighbor, neighbor) in self.first_steps() {
            let mut prev = self.starting_loc;
            let mut prev_to_curr = start_to_neighbor;
            let mut curr = neighbor;

            let mut path = Path::new(self, self.starting_loc, start_to_neighbor, neighbor);

            while let Some((curr_to_next, next)) =
                self.find_next(prev, prev_to_curr, curr, &path, true)
            {
                path.extend(curr, next);
                if let Tile::Start = self.at(next) {
                    return path;
                }

                prev = curr;
                prev_to_curr = curr_to_next;
                curr = next;
            }
            println!("{neighbor:?} rejected");
        }

        panic!("No path found from start back to start!");
    }
}

#[test]
fn simple_square() {
    static SIMPLE_SQUARE1: &str = "
.....
.S-7.
.|.|.
.L-J.
.....";
    let sketch = Sketch::new(SIMPLE_SQUARE1);
    let path = sketch.compute_path();

    let dist = path.greatest_distance_from_start();
    println!("Greatest distance (simple 1): {dist}");
    assert_eq!(dist, 4);

    let count = path.count_inside();
    println!("Inside count: {count}");
    assert_eq!(count, 1);
}

#[test]
fn simple_square2() {
    static SIMPLE_SQUARE2: &str = "
.....
.F-S.
.L-J.
.....";
    let sketch = Sketch::new(SIMPLE_SQUARE2);
    let path = sketch.compute_path();
    let dist = path.greatest_distance_from_start();
    println!("Greatest distance (simple 2): {dist}");
    assert_eq!(dist, 3);

    let count = path.count_inside();
    println!("Inside count: {count}");
    assert_eq!(count, 0);
}

#[test]
fn simple_square3() {
    static SIMPLE_SQUARE3: &str = "
.....
.F-7.
.|.|.
.|.|.
.L-S.
.....";
    let sketch = Sketch::new(SIMPLE_SQUARE3);
    let path = sketch.compute_path();
    let dist = path.greatest_distance_from_start();
    println!("Greatest distance (simple 3): {dist}");
    assert_eq!(dist, 5);

    let count = path.count_inside();
    println!("Inside count: {count}");
    assert_eq!(count, 2);
}

#[test]
fn simple_square4() {
    static SIMPLE_SQUARE4: &str = "
.....
.F-7.
.|.|.
.L-S.
.....";
    let sketch = Sketch::new(SIMPLE_SQUARE4);
    let path = sketch.compute_path();
    let dist = path.greatest_distance_from_start();
    println!("Greatest distance (simple 4): {dist}");
    assert_eq!(dist, 4);

    let count = path.count_inside();
    println!("Inside count: {count}");
    assert_eq!(count, 1);
}

#[test]
fn bug1() {
    static BUG1: &str = "
F---7.
|F7.|.
|||.|.
LJ|FS.
..LJ..";
    let sketch = Sketch::new(BUG1);
    let path = sketch.compute_path();
    let dist = path.greatest_distance_from_start();
    println!("Greatest distance (bug 1): {dist}");
    assert_eq!(dist, 10);

    let count = path.count_inside();
    println!("Inside count: {count}");
    assert_eq!(count, 2);
}

#[test]
fn bug2() {
    static BUG2: &str = "
L-7|J......-.----
7FJL-.F-----7-|F-
LJF--.||F-7.|.F7J
7FJF7.|FJFJ.|.|L7
|L-JL.|L7|FSJ.|FJ
JF---.L-J||...||-
7L-7F.-L.LJ|.LJL7
JF-J||JF...J.-7FJ
";
    let sketch = Sketch::new(BUG2);
    let path = sketch.compute_path();
    let dist = path.greatest_distance_from_start();
    println!("Greatest distance (bug 2): {dist}");
    assert_eq!(dist, 16);

    let count = path.count_inside();
    println!("Inside count: {count}");
    assert_eq!(count, 3);
}

#[test]
fn complex_loop() {
    static COMPLEX_LOOP: &str = "
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
    let sketch = Sketch::new(COMPLEX_LOOP);
    let path = sketch.compute_path();
    let dist = path.greatest_distance_from_start();
    println!("Greatest distance (complex): {dist}");
    assert_eq!(dist, 8);

    let count = path.count_inside();
    println!("Inside count: {count}");
    assert_eq!(count, 1);
}

#[test]
fn inside_area_tests() {
    static TESTS: &[(&str, usize)] = &[
        (
            "
.F-7....
.S-L--7.
.L----J.
    ",
            1,
        ),
        (
            "
.F-7....
FS-L--7.
L-----J.
    ",
            1,
        ),
        (
            "
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
",
            4,
        ),
        (
            "
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
",
            4,
        ),
        (
            "
..........
.S------7.
.|.F--7.|.
.|FJ..L7|.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
",
            6,
        ),
        (
            "
..........
.F------7.
.S.F--7.|.
.|FJ..L7|.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
",
            6,
        ),
        (
            "
F-7.
|.|.
LSL7
.|.|
.L-J
",
            2,
        ),
        (
            "
F-7.
|F|.
LSL7
.|-|
.L-J
",
            2,
        ),
        (
            "
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
",
            8,
        ),
        (
            "
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
",
            10,
        ),
        (
            "
JJ|F-7.F.F7J
7.FJFJ.F7|L7
7-L7|FS|||FJ
J|FJ|||||||-
7-L7LJ||LJL7
|JFL--JL-7FJ
",
            0,
        ),
        (
            "
....
....
F7..
|L-7
S..|
L--J
",
            2,
        ),
        (
            "
....
....
F7..
||F7
SLJ|
L--J
",
            0,
        ),
        (
            "
....
S--7
L--J
",
            0,
        ),
    ];

    for &(diagram, expected) in TESTS {
        println!("{diagram}");

        let sketch = Sketch::new(diagram);
        let path = sketch.compute_path();

        let count = path.count_inside();
        println!("Inside count: {count}");
        assert_eq!(count, expected);
    }
}

#[test]
fn inside_area_1() {
    let diagram = "
.F7.
SJL7
L--J
";
    let expected = 0;

    println!("{diagram}");

    let sketch = Sketch::new(diagram);
    let path = sketch.compute_path();

    let count = path.count_inside();
    println!("Inside count: {count}");
    assert_eq!(count, expected);
}

#[test]
fn inside_area_2() {
    let diagram = "
.F7.
FJL7
S--J
";
    let expected = 0;

    println!("{diagram}");

    let sketch = Sketch::new(diagram);
    let path = sketch.compute_path();

    let count = path.count_inside();
    println!("Inside count: {count}");
    assert_eq!(count, expected);
}

#[test]
fn inside_area_3() {
    let diagram = "
.F7.
FJL7
S-FJ
L-J.
";
    let expected = 1;

    println!("{diagram}");

    let sketch = Sketch::new(diagram);
    let path = sketch.compute_path();

    let count = path.count_inside();
    println!("Inside count: {count}");
    assert_eq!(count, expected);
}

#[test]
fn inside_area_4() {
    let diagram = "
.F7.
FJL7
|-FJ
S-J.
";
    let expected = 1;

    println!("{diagram}");

    let sketch = Sketch::new(diagram);
    let path = sketch.compute_path();

    let count = path.count_inside();
    println!("Inside count: {count}");
    assert_eq!(count, expected);
}

#[test]
fn inside_area_5() {
    let diagram = "
.FS.
FJL7
|-.|
L--J
";
    let expected = 2;

    println!("{diagram}");

    let sketch = Sketch::new(diagram);
    let path = sketch.compute_path();

    let count = path.count_inside();
    println!("Inside count: {count}");
    assert_eq!(count, expected);
}

#[test]
fn inside_area_6() {
    let diagram = "
.F7.
FJL7
|-FJ
S-J.
";
    let expected = 1;

    println!("{diagram}");

    let sketch = Sketch::new(diagram);
    let path = sketch.compute_path();

    let count = path.count_inside();
    println!("Inside count: {count}");
    assert_eq!(count, expected);
}

#[test]
fn inside_area_test_both_prev_and_curr() {
    let diagram = "
.F7F7..
.|||L-7
.|LJF-J
.L7.L-7
.FJF7FJ
.L7|||.
..LJLJ.
";
    let expected = 1;

    // prev YES
    // curr NO

    fn add_s(s: &str) -> impl Iterator<Item = String> {
        let mut res = vec![];

        for (i, c) in s.char_indices() {
            if ['F', '-', '7', '|', '-', 'J', 'L'].contains(&c) {
                let before = &s[0..i];
                let after = &s[i + c.len_utf8()..];

                let mut replaced = String::from(before);
                replaced += "S";
                replaced += after;

                res.push(replaced);
            }
        }

        res.into_iter()
    }

    for diagram in add_s(diagram) {
        println!("{diagram}");

        let sketch = Sketch::new(&diagram);
        let path = sketch.compute_path();

        let count = path.count_inside();
        println!("Inside count: {count}");
        assert_eq!(count, expected);
    }
}

#[test]
fn inside_area_many_empty_h() {
    let diagram = "
F7F7
|LJ|
|F7|
LJLJ
";
    let expected = 0;

    fn add_s(s: &str) -> impl Iterator<Item = String> {
        let mut res = vec![];

        for (i, c) in s.char_indices() {
            if ['F', '-', '7', '|', '-', 'J', 'L'].contains(&c) {
                let before = &s[0..i];
                let after = &s[i + c.len_utf8()..];

                let mut replaced = String::from(before);
                replaced += "S";
                replaced += after;

                res.push(replaced);
            }
        }

        res.into_iter()
    }

    for diagram in add_s(diagram) {
        println!("{diagram}");

        let sketch = Sketch::new(&diagram);
        let path = sketch.compute_path();

        let count = path.count_inside();
        println!("Inside count: {count}");
        assert_eq!(count, expected);
    }
}

#[test]
fn inside_area_many_empty_h2() {
    let diagram = "
F7F7
|LJ|
|.FJ
L-J.
";
    let expected = 1;

    fn add_s(s: &str) -> impl Iterator<Item = String> {
        let mut res = vec![];

        for (i, c) in s.char_indices() {
            if ['F', '-', '7', '|', '-', 'J', 'L'].contains(&c) {
                let before = &s[0..i];
                let after = &s[i + c.len_utf8()..];

                let mut replaced = String::from(before);
                replaced += "S";
                replaced += after;

                res.push(replaced);
            }
        }

        res.into_iter()
    }

    for diagram in add_s(diagram) {
        println!("{diagram}");

        let sketch = Sketch::new(&diagram);
        let path = sketch.compute_path();

        let count = path.count_inside();
        println!("Inside count: {count}");
        assert_eq!(count, expected);
    }
}

#[test]
fn inside_area_many_empty_u() {
    let diagram = "
F7F7
||||
|LJ|
L--J
";
    let expected = 0;

    fn add_s(s: &str) -> impl Iterator<Item = String> {
        let mut res = vec![];

        for (i, c) in s.char_indices() {
            if ['F', '-', '7', '|', '-', 'J', 'L'].contains(&c) {
                let before = &s[0..i];
                let after = &s[i + c.len_utf8()..];

                let mut replaced = String::from(before);
                replaced += "S";
                replaced += after;

                res.push(replaced);
            }
        }

        res.into_iter()
    }

    for diagram in add_s(diagram) {
        println!("{diagram}");

        let sketch = Sketch::new(&diagram);
        let path = sketch.compute_path();

        let count = path.count_inside();
        println!("Inside count: {count}");
        assert_eq!(count, expected);
    }
}

#[test]
fn inside_area_many() {
    let patterns = [
        "
F-7..
|.L-7
|...|
L---J
",
        "
.F-7.
FJ.L7
|...|
L---J
",
        "
..F-7
F-J.|
|...|
L---J
",
        "
...F-7
F--J.|
|...FJ
L---J.
",
        "
...F-7
F--J.|
|...FJ
L---J.
",
        "
F----7
|....|
L----J
",
        "
F---7.
|...L7
L--7.|
...L-J
",
        "
F---7
|...|
L-7.|
..L-J
",
        "
F---7
|...|
L7.FJ
.L-J.
",
    ];
    let expected = 4;

    fn add_s(s: &str) -> impl Iterator<Item = String> {
        let mut res = vec![];

        for (i, c) in s.char_indices() {
            if ['F', '-', '7', '|', '-', 'J', 'L'].contains(&c) {
                let before = &s[0..i];
                let after = &s[i + c.len_utf8()..];

                let mut replaced = String::from(before);
                replaced += "S";
                replaced += after;

                res.push(replaced);
            }
        }

        res.into_iter()
    }

    for pattern in patterns {
        for diagram in add_s(pattern) {
            println!("{diagram}");

            let sketch = Sketch::new(&diagram);
            let path = sketch.compute_path();

            let count = path.count_inside();
            println!("Inside count: {count}");
            assert_eq!(count, expected);
        }
    }
}

fn main() {
    static INPUT: &str = include_str!("../input");

    // Part 1.
    println!("Part 1:");
    let sketch = Sketch::new(INPUT);
    let path = sketch.compute_path();
    let dist = path.greatest_distance_from_start();
    println!("Greatest distance: {dist}");
    assert_eq!(dist, 6773);

    // Path 2.
    println!("Part 2:");
    let count = path.count_inside();
    println!("Inside count: {count}");
    assert_eq!(count, 493);
}
