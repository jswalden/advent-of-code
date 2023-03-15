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

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Vertical {
    Up,
    Down,
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

struct BoardMap {
    tiles: Vec<Vec<Tile>>,
}

impl Debug for BoardMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.tiles {
            write!(f, "{line:?}")?;
        }

        Ok(())
    }
}

impl BoardMap {
    fn new(s: &str) -> BoardMap {
        let mut tiles = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        ' ' => Tile::Absent,
                        '.' => Tile::Open,
                        '#' => Tile::Wall,
                        c => panic!("unexpected tile: {c:?}"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let max_width = tiles.iter().map(|v| v.len()).max().unwrap();

        for v in &mut tiles {
            v.resize(max_width, Tile::Absent);
        }

        BoardMap { tiles }
    }

    fn tile_at(&self, x: usize, y: usize) -> Tile {
        self.tiles[y][x]
    }

    fn move_y(&self, x: usize, y: usize, vert: Vertical) -> Option<usize> {
        assert!(
            self.tile_at(x, y) == Tile::Open,
            "must have valid starting position"
        );

        let mut current_y = y;
        loop {
            let cand_y = match vert {
                Vertical::Up => {
                    if current_y == 0 {
                        self.tiles.len() - 1
                    } else {
                        current_y - 1
                    }
                }
                Vertical::Down => {
                    if current_y == self.tiles.len() - 1 {
                        0
                    } else {
                        current_y + 1
                    }
                }
            };
            match self.tile_at(x, cand_y) {
                Tile::Wall => return None,
                Tile::Open => return Some(cand_y),
                Tile::Absent => current_y = cand_y,
            }
        }
    }

    fn move_x(&self, x: usize, y: usize, horiz: Horizontal) -> Option<usize> {
        assert!(
            self.tile_at(x, y) == Tile::Open,
            "must have valid starting position"
        );

        let mut current_x = x;
        loop {
            let cand_x = match horiz {
                Horizontal::Left => {
                    if current_x == 0 {
                        self.tiles[y].len() - 1
                    } else {
                        current_x - 1
                    }
                }
                Horizontal::Right => {
                    if current_x == self.tiles[y].len() - 1 {
                        0
                    } else {
                        current_x + 1
                    }
                }
            };
            match self.tile_at(cand_x, y) {
                Tile::Wall => return None,
                Tile::Open => return Some(cand_x),
                Tile::Absent => current_x = cand_x,
            }
        }
    }

    fn compute_move(
        &self,
        (start_x, start_y): (usize, usize),
        dir: Direction,
        dist: Dist,
    ) -> (usize, usize) {
        assert!(
            self.tile_at(start_x, start_y) == Tile::Open,
            "must have valid starting position"
        );

        //println!("computing move from (({start_x}, {start_y}), {dir:?}) by {dist}");

        let (mut pos_x, mut pos_y) = (start_x, start_y);
        for _ in 0..dist {
            match dir {
                Direction::Horizontal(h) => {
                    if let Some(new_x) = self.move_x(pos_x, pos_y, h) {
                        pos_x = new_x;
                    } else {
                        break;
                    }
                }
                Direction::Vertical(v) => {
                    if let Some(new_y) = self.move_y(pos_x, pos_y, v) {
                        pos_y = new_y;
                    } else {
                        break;
                    }
                }
            };
            //println!("moved to ({pos_x}, {pos_y})...");
        }

        //println!("move complete");
        (pos_x, pos_y)
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

fn parse_board_map(s: &'static str) -> BoardMap {
    BoardMap::new(s)
}

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

fn parse_path_description(desc: &'static str) -> impl Iterator<Item = Movement> {
    MoveIter::new(desc)
}

fn parse_input(input: &'static str) -> (BoardMap, impl Iterator<Item = Movement>) {
    {
        let mut iter = input.split("\n\n");

        let board_str = iter.next().expect("board map");
        let path_desc = iter.next().expect("path description").trim();

        (
            parse_board_map(board_str),
            parse_path_description(path_desc),
        )
    }
}

#[test]
fn boards() {
    assert_eq!(
        parse_board_map(
            "  ..#
..#.##
#....."
        )
        .tiles,
        vec![
            vec![
                Tile::Absent,
                Tile::Absent,
                Tile::Open,
                Tile::Open,
                Tile::Wall,
                Tile::Absent
            ],
            vec![
                Tile::Open,
                Tile::Open,
                Tile::Wall,
                Tile::Open,
                Tile::Wall,
                Tile::Wall
            ],
            vec![
                Tile::Wall,
                Tile::Open,
                Tile::Open,
                Tile::Open,
                Tile::Open,
                Tile::Open
            ]
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
    path: &mut impl Iterator<Item = Movement>,
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
                (x, y) = board_map.compute_move((x, y), dir, dist);
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

    let (board_map, mut path) = parse_input(INPUT);

    // Part 1.
    {
        let ((x, y), dir) = find_final_position(&board_map, &mut path);
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
        let ((x, y), dir) = find_final_position(&board_map, &mut path);
        println!("x: {x}, y: {y}, dir: {dir:?}");
        assert_eq!(
            (x + 1, y + 1, dir),
            (8, 6, Direction::Horizontal(Horizontal::Right)),
        );

        let password = compute_password(x, y, dir);
        println!("Password: {password}");
        assert_eq!(password, 6_032);
    }
}

fn main() {
    static INPUT: &str = include_str!("../input");

    let (board_map, mut path) = parse_input(INPUT);
    assert_eq!(
        board_map.at_start(),
        ((50, 0), Direction::Horizontal(Horizontal::Right))
    );

    // Part 1.
    {
        let ((x, y), dir) = find_final_position(&board_map, &mut path);
        println!("x: {x}, y: {y}, dir: {dir:?}");

        let password = compute_password(x, y, dir);
        println!("Password: {password}");
        assert_eq!(password, 144_244);
    }

    // Part 2.
    {
        let ((x, y), dir) = find_final_position(&board_map, &mut path);
        println!("x: {x}, y: {y}, dir: {dir:?}");

        let password = compute_password(x, y, dir);
        println!("Password: {password}");
        assert_eq!(password, 144_244);
    }
}
