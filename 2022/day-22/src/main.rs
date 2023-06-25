use std::fmt::Debug;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Absent,
    Open,
    Wall,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Absent => ' ',
            Self::Open => '.',
            Self::Wall => '#',
        };
        write!(f, "{c}")
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Turn {
    Left,
    Right,
}

type Dist = u32;

#[derive(Debug, PartialEq, Eq)]
enum Movement {
    Forward(Dist),
    Turn(Turn),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Horizontal {
    Left,
    Right,
}

impl Horizontal {
    fn step(&self) -> isize {
        match self {
            Horizontal::Left => -1,
            Horizontal::Right => 1,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Vertical {
    Up,
    Down,
}

impl Vertical {
    fn step(&self) -> isize {
        match self {
            Vertical::Up => -1,
            Vertical::Down => 1,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Horizontal(Horizontal),
    Vertical(Vertical),
}

impl Direction {
    fn apply_turn(&self, turn: Turn) -> Direction {
        match (turn, *self) {
            (Turn::Left, Direction::Horizontal(Horizontal::Left)) => {
                Direction::Vertical(Vertical::Down)
            }
            (Turn::Left, Direction::Horizontal(Horizontal::Right)) => {
                Direction::Vertical(Vertical::Up)
            }

            (Turn::Left, Direction::Vertical(Vertical::Up)) => {
                Direction::Horizontal(Horizontal::Left)
            }

            (Turn::Left, Direction::Vertical(Vertical::Down)) => {
                Direction::Horizontal(Horizontal::Right)
            }

            (Turn::Right, Direction::Horizontal(Horizontal::Left)) => {
                Direction::Vertical(Vertical::Up)
            }
            (Turn::Right, Direction::Horizontal(Horizontal::Right)) => {
                Direction::Vertical(Vertical::Down)
            }

            (Turn::Right, Direction::Vertical(Vertical::Up)) => {
                Direction::Horizontal(Horizontal::Right)
            }
            (Turn::Right, Direction::Vertical(Vertical::Down)) => {
                Direction::Horizontal(Horizontal::Left)
            }
        }
    }

    fn to_number(&self) -> u8 {
        match *self {
            Direction::Vertical(Vertical::Up) => 3,
            Direction::Vertical(Vertical::Down) => 1,
            Direction::Horizontal(Horizontal::Left) => 2,
            Direction::Horizontal(Horizontal::Right) => 0,
        }
    }
}

#[derive(Copy, Clone)]
enum Folding {
    Unfolded,
    Cube,
}

struct BoardMap {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    side_width: usize,
}

impl Debug for BoardMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.tiles {
            write!(f, "{line:?}")?;
        }

        Ok(())
    }
}

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

impl BoardMap {
    fn new(s: &'static str, side_width: usize) -> BoardMap {
        let (mut tiles, width) = s
            .lines()
            .map(|line| {
                let tiles = line
                    .chars()
                    .map(|c| match c {
                        ' ' => Tile::Absent,
                        '.' => Tile::Open,
                        '#' => Tile::Wall,
                        c => panic!("unexpected tile: {c:?}"),
                    })
                    .collect::<Vec<_>>();
                let tiles_len = tiles.len();
                (tiles, tiles_len)
            })
            .fold(
                (vec![], 0),
                |(mut tiles_list, max_width): (Vec<Vec<Tile>>, usize),
                 (tiles, width): (Vec<Tile>, usize)| {
                    tiles_list.push(tiles);
                    (tiles_list, max_width.max(width))
                },
            );

        for v in &mut tiles {
            v.resize(width, Tile::Absent);
        }

        let height = tiles.len();

        assert!(width % side_width == 0);
        assert!(height % side_width == 0);
        BoardMap {
            tiles,
            width,
            height,
            side_width,
        }
    }

    fn tile_at(&self, x: usize, y: usize) -> Tile {
        self.tiles[y][x]
    }

    fn step(
        &self,
        x: usize,
        y: usize,
        dir: Direction,
        folding: Folding,
    ) -> ((usize, usize), Direction) {
        // First try moving straight in the current direction.
        let (trial_x, trial_y) = match dir {
            Direction::Horizontal(h) => {
                let new_x = wrapping_add(x, h.step(), self.tiles[y].len());
                (new_x, y)
            }
            Direction::Vertical(v) => {
                let new_y = wrapping_add(y, v.step(), self.tiles.len());
                (x, new_y)
            }
        };

        match (self.tile_at(trial_x, trial_y), folding) {
            (_, Folding::Unfolded) | (Tile::Wall | Tile::Open, _) => {
                // If we're not folding a cube or we're not walking off the cube
                // sides, return the trial location and previous direction.
                ((trial_x, trial_y), dir)
            }
            (Tile::Absent, Folding::Cube) => {
                // If we're walking off the side while folding a cube, compute
                // the location x/y fold up against and the altered direction.
                let (xrem, yrem) = (x % self.side_width, y % self.side_width);

                match dir {
                    Direction::Vertical(v) => {
                        let x_comp = self.side_width - 1 - yrem;

                        let (x_lower, x_higher) = {
                            let x_start = x - x_comp;

                            let x_lower = if x_start == 0 {
                                self.width
                            } else {
                                x_start - 1
                            };
                            let x_higher = x_start + self.side_width;

                            (x_lower, x_higher)
                        };

                        // Try x_higher side.
                    }
                    Direction::Horizontal(h) => {
                        let y_comp = self.side_width - 1 - xrem;
                    }
                }
                todo!();
            }
        }
    }

    fn move_one(
        &self,
        x: usize,
        y: usize,
        dir: Direction,
        folding: Folding,
    ) -> Option<((usize, usize), Direction)> {
        assert!(self.tile_at(x, y) == Tile::Open, "x/y must be valid");

        let (mut current_x, mut current_y, mut current_dir) = (x, y, dir);
        loop {
            let ((cand_x, cand_y), cand_dir) =
                self.step(current_x, current_y, current_dir, folding);
            match self.tile_at(cand_x, cand_y) {
                Tile::Wall => return None,
                Tile::Open => return Some(((cand_x, cand_y), cand_dir)),
                Tile::Absent => {
                    ((current_x, current_y), current_dir) = ((cand_x, cand_y), cand_dir)
                }
            }
        }
    }

    fn compute_move(
        &self,
        (start_x, start_y): (usize, usize),
        dir: Direction,
        dist: Dist,
        folding: Folding,
    ) -> ((usize, usize), Direction) {
        assert!(
            self.tile_at(start_x, start_y) == Tile::Open,
            "must have valid starting position"
        );

        //println!("computing move from (({start_x}, {start_y}), {dir:?}) by {dist}");

        let (mut pos_x, mut pos_y) = (start_x, start_y);
        let mut curr_dir = dir;
        for _ in 0..dist {
            if let Some(((new_x, new_y), new_dir)) = self.move_one(pos_x, pos_y, curr_dir, folding)
            {
                (pos_x, pos_y) = (new_x, new_y);
                curr_dir = new_dir;
            } else {
                break;
            }
            //println!("moved to ({pos_x}, {pos_y})...");
        }

        //println!("move complete");
        ((pos_x, pos_y), dir)
    }

    fn at_start(&self) -> ((usize, usize), Direction) {
        let x = self.tiles[0]
            .iter()
            .enumerate()
            .find(|(_i, &tile)| tile == Tile::Open)
            .expect("must have open tile")
            .0;
        ((x, 0), Direction::Horizontal(Horizontal::Right))
    }
}

fn parse_board_map(s: &'static str, side_width: usize) -> BoardMap {
    BoardMap::new(s, side_width)
}

#[derive(Clone)]
struct MoveIter {
    remaining: Peekable<Chars<'static>>,
}

impl MoveIter {
    fn new(desc: &'static str) -> MoveIter {
        MoveIter {
            remaining: desc.chars().peekable(),
        }
    }
}

impl Iterator for MoveIter {
    type Item = Movement;

    fn next(&mut self) -> Option<Movement> {
        let c = self.remaining.next()?;
        if c == 'L' {
            return Some(Movement::Turn(Turn::Left));
        }
        if c == 'R' {
            return Some(Movement::Turn(Turn::Right));
        }

        let mut dist = c.to_digit(10).unwrap();
        while let Some(c) = self.remaining.peek().and_then(|c| c.to_digit(10)) {
            self.remaining.next().unwrap();
            dist = dist * 10 + c;
        }
        Some(Movement::Forward(dist))
    }
}

fn parse_path_description(desc: &'static str) -> impl Iterator<Item = Movement> + Clone {
    MoveIter::new(desc)
}

fn parse_input(
    input: &'static str,
    side_width: usize,
) -> (BoardMap, impl Iterator<Item = Movement> + Clone) {
    {
        let mut iter = input.split("\n\n");

        let board_str = iter.next().expect("board map");
        let path_desc = iter.next().expect("path description").trim();

        (
            parse_board_map(board_str, side_width),
            parse_path_description(path_desc),
        )
    }
}

#[test]
fn boards() {
    assert_eq!(
        parse_board_map(
            "  ..#.
  #.
  ..
  ##
  .#
  ..
#..#
..#.
",
            2
        )
        .tiles,
        vec![
            vec![
                Tile::Absent,
                Tile::Absent,
                Tile::Open,
                Tile::Open,
                Tile::Wall,
                Tile::Open
            ],
            vec![
                Tile::Absent,
                Tile::Absent,
                Tile::Wall,
                Tile::Open,
                Tile::Absent,
                Tile::Absent
            ],
            vec![
                Tile::Absent,
                Tile::Absent,
                Tile::Open,
                Tile::Open,
                Tile::Absent,
                Tile::Absent
            ],
            vec![
                Tile::Absent,
                Tile::Absent,
                Tile::Wall,
                Tile::Wall,
                Tile::Absent,
                Tile::Absent
            ],
            vec![
                Tile::Absent,
                Tile::Absent,
                Tile::Open,
                Tile::Wall,
                Tile::Absent,
                Tile::Absent
            ],
            vec![
                Tile::Absent,
                Tile::Absent,
                Tile::Open,
                Tile::Open,
                Tile::Absent,
                Tile::Absent
            ],
            vec![
                Tile::Wall,
                Tile::Open,
                Tile::Open,
                Tile::Wall,
                Tile::Absent,
                Tile::Absent
            ],
            vec![
                Tile::Open,
                Tile::Open,
                Tile::Wall,
                Tile::Open,
                Tile::Absent,
                Tile::Absent
            ],
        ]
    );
}

#[test]
fn paths() {
    assert_eq!(
        parse_path_description("1L203R3L").collect::<Vec<_>>(),
        vec![
            Movement::Forward(1),
            Movement::Turn(Turn::Left),
            Movement::Forward(203),
            Movement::Turn(Turn::Right),
            Movement::Forward(3),
            Movement::Turn(Turn::Left),
        ]
    );
}

fn find_final_position(
    board_map: &BoardMap,
    path: impl Iterator<Item = Movement>,
    folding: Folding,
) -> ((usize, usize), Direction) {
    let ((mut x, mut y), mut dir) = board_map.at_start();

    let mut step = 0;
    macro_rules! print_location {
        () => {
            step += 1;
            if step < 10 {
                println!("at ({x}, {y}) direction {dir:?}");
            }
        };
    }

    print_location!();
    for movement in path {
        match movement {
            Movement::Forward(dist) => {
                ((x, y), dir) = board_map.compute_move((x, y), dir, dist, folding);
            }
            Movement::Turn(turn) => dir = dir.apply_turn(turn),
        };
        print_location!();
    }

    ((x, y), dir)
}

fn compute_password(x: usize, y: usize, dir: Direction) -> u32 {
    (1000 * (y + 1)) as u32 + (4 * (x + 1)) as u32 + dir.to_number() as u32
}

#[test]
fn example() {
    static INPUT: &str = "        ...#
    .#..
    #...
    ....
...#.......#
........#...
..#....#....
..........#.
    ...#....
    .....#..
    .#......
    ......#.

10R5L5R10L4R5L5";

    const SIDE_WIDTH: usize = 4;

    let (board_map, path) = parse_input(INPUT, SIDE_WIDTH);

    // Part 1.
    {
        let ((x, y), dir) = find_final_position(&board_map, path.clone(), Folding::Unfolded);
        println!("x: {x}, y: {y}, dir: {dir:?}");
        assert_eq!(
            (x + 1, y + 1, dir),
            (8, 6, Direction::Horizontal(Horizontal::Right)),
        );

        let password = compute_password(x, y, dir);
        println!("Password: {password}");
        assert_eq!(password, 6_032);
    }

    // Part 2.
    {
        let ((x, y), dir) = find_final_position(&board_map, path, Folding::Cube);
        println!("x: {x}, y: {y}, dir: {dir:?}");
        assert_eq!(
            (x + 1, y + 1, dir),
            (8, 6, Direction::Horizontal(Horizontal::Right)),
        );

        let password = compute_password(x, y, dir);
        println!("Password: {password} (should equal 5031)");
        //assert_eq!(password, 5_031);
    }
}

fn main() {
    static INPUT: &str = include_str!("../input");

    const SIDE_WIDTH: usize = 50;

    let (board_map, path) = parse_input(INPUT, SIDE_WIDTH);
    assert_eq!(
        board_map.at_start(),
        ((50, 0), Direction::Horizontal(Horizontal::Right))
    );

    // Part 1.
    {
        let ((x, y), dir) = find_final_position(&board_map, path.clone(), Folding::Unfolded);
        println!("x: {x}, y: {y}, dir: {dir:?}");

        let password = compute_password(x, y, dir);
        println!("Password: {password}");
        assert_eq!(password, 144_244);
    }

    // Part 2.
    {
        let ((x, y), dir) = find_final_position(&board_map, path, Folding::Cube);
        println!("x: {x}, y: {y}, dir: {dir:?}");

        let password = compute_password(x, y, dir);
        println!("Password: {password}");
        assert_eq!(password, 144_244);
    }
}
