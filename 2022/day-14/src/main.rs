use std::collections::HashMap;
use std::collections::HashSet;

const AIR: u8 = b'.';
const SAND: u8 = b'o';
const ROCK: u8 = b'#';
const SOURCE: u8 = b'+';

const STARTING_ROW: usize = 0;
const STARTING_COLUMN: usize = 500;

#[derive(Copy, PartialEq, Eq, Hash, Clone, Ord, PartialOrd)]
struct CaveCoord(usize);

#[derive(Copy, Clone)]
struct CaveCoords(CaveCoord, CaveCoord);

struct CaveExtent {
    greatest_row: CaveCoord,
    smallest_column: CaveCoord,
    greatest_column: CaveCoord,
}

#[derive(Copy, Clone)]
struct GridCoords(usize, usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Floor {
    None,
    Infinite,
}

struct Grid {
    grid: Vec<u8>,
    width: usize,
    height: usize,
    source: GridCoords,
    floor: Floor,
}

type RocksSet = HashMap<CaveCoord, HashSet<CaveCoord>>;

impl Grid {
    fn new(rocks: &RocksSet, cave_extent: &CaveExtent, floor: Floor) -> Grid {
        let greatest_row = cave_extent.greatest_row.0;

        let (height, smallest_column, greatest_column) = match floor {
            Floor::None => {
                let height = greatest_row + 1;
                let smallest_column = cave_extent.smallest_column.0;
                let greatest_column = cave_extent.greatest_column.0;
                (height, smallest_column, greatest_column)
            }
            Floor::Infinite => {
                let height = greatest_row + 2 + 1;

                // Smallest column if the source is the very top of a pyramid
                // shape of sand.
                let smallest_column_narrow_cave = STARTING_COLUMN - (greatest_row + 2);

                // Greatest column if the source is the very top of a pyramid
                // shape of sand.
                let greatest_column_narrow_cave = STARTING_COLUMN + (greatest_row + 2);

                let smallest_column =
                    smallest_column_narrow_cave.min(cave_extent.smallest_column.0);
                let greatest_column =
                    greatest_column_narrow_cave.max(cave_extent.greatest_column.0);

                (height, smallest_column, greatest_column)
            }
        };

        let width = greatest_column - smallest_column + 1;
        let source = GridCoords(STARTING_ROW, STARTING_COLUMN - smallest_column);

        let mut g = Grid {
            grid: vec![AIR; width * height],
            width,
            height,
            source,
            floor,
        };

        for (row, cols) in rocks {
            for col in cols {
                let coords = g.translate(CaveCoords(*row, *col));
                *g.at_mut(coords) = ROCK;
            }
        }

        if let Floor::Infinite = floor {
            let floor_row = height - 1;
            for col in 0..width {
                let floor_coord = GridCoords(floor_row, col);
                assert!(*g.at(floor_coord) == AIR);
                *g.at_mut(floor_coord) = ROCK;
            }
        }

        assert_eq!(*g.at(g.source), AIR);
        *g.at_mut(g.source) = SOURCE;

        g
    }

    fn _draw(&self) {
        let mut i = 0;
        let grid = &self.grid;
        let width = self.width;
        while i < grid.len() {
            for j in 0..width {
                print!("{}", grid[i + j] as char);
            }
            print!("\n");
            i += width;
        }
        if let Floor::Infinite = self.floor {
            for _ in 0..width {
                print!("{}", ROCK);
            }
            print!("\n");
        }
    }

    fn translate(&self, coords: CaveCoords) -> GridCoords {
        let row = (coords.0).0;
        assert!(row < self.height);

        let col = (coords.1).0 + self.source.1 - STARTING_COLUMN;
        assert!(col < self.width);

        GridCoords(row, col)
    }

    fn at(&self, coords: GridCoords) -> &u8 {
        let GridCoords(row, col) = coords;
        assert!(row < self.height);
        assert!(col < self.width);
        &self.grid[row * self.width + col]
    }

    fn at_mut(&mut self, coords: GridCoords) -> &mut u8 {
        let GridCoords(row, col) = coords;
        assert!(row < self.height);
        assert!(col < self.width);
        &mut self.grid[row * self.width + col]
    }

    fn add_sand(&mut self) -> usize {
        let res = self.add_sand_helper();
        if let Floor::None = self.floor {
            assert!(res <= 1);
        }
        res
    }

    fn add_sand_helper(&mut self) -> usize {
        let mut grain_coords = self.source;
        if *self.at(grain_coords) != SOURCE {
            assert_eq!(self.floor, Floor::Infinite);
            return 0;
        }

        let Grid { height, width, .. } = *self;

        loop {
            // Square at start of loop is always empty.
            if let Floor::None = self.floor {
                assert!([AIR, SOURCE].contains(self.at(grain_coords)));
            }

            // Directly beneath.
            if grain_coords.0 + 1 >= height {
                assert_eq!(grain_coords.0 + 1, height);
                return 0;
            }

            grain_coords.0 += 1;
            if *self.at(grain_coords) == AIR {
                continue;
            }

            // To left of beneath.
            if grain_coords.1 == 0 {
                return 0;
            }

            grain_coords.1 -= 1;
            if *self.at(grain_coords) == AIR {
                continue;
            }

            // To right of beneath.
            if grain_coords.1 + 2 == width {
                return 0;
            }

            grain_coords.1 += 2;
            if *self.at(grain_coords) == AIR {
                continue;
            }

            // Reset to original coordinate before filling with sand.
            grain_coords.0 -= 1;
            grain_coords.1 -= 1;

            // Fill square with sand.
            *self.at_mut(grain_coords) = SAND;
            return 1;
        }
    }

    fn add_all_sand(&mut self) -> usize {
        let mut count = 0;
        loop {
            let grains_added = self.add_sand();
            if grains_added == 0 {
                break;
            }
            count += grains_added;
        }
        count
    }
}

fn parse_to_grid(input: &str, floor: Floor) -> Grid {
    let mut rocks = RocksSet::new();

    let mut greatest_row = 0;
    let mut smallest_column = STARTING_COLUMN;
    let mut greatest_column = STARTING_COLUMN;

    {
        let mut add_rock = |CaveCoords(row, col): CaveCoords| {
            greatest_row = greatest_row.max(row.0);
            smallest_column = smallest_column.min(col.0);
            greatest_column = greatest_column.max(col.0);

            if let Some(row) = rocks.get_mut(&row) {
                row.insert(col);
            } else {
                let mut h = HashSet::new();
                h.insert(col);
                rocks.insert(row, h);
            }
        };

        for line in input.lines() {
            for (c1, c2) in itertools::Itertools::tuple_windows(line.split(" -> ").map(|coord| {
                let (col, row) = coord.split_once(',').expect("coords");
                CaveCoords(
                    CaveCoord(row.parse().expect("row")),
                    CaveCoord(col.parse().expect("col")),
                )
            })) {
                if c1.0 == c2.0 {
                    let start = c1.1.min(c2.1);
                    let end = c1.1.max(c2.1);
                    for col in start.0..=end.0 {
                        add_rock(CaveCoords(c1.0, CaveCoord(col)));
                    }
                } else if c1.1 == c2.1 {
                    let start = c1.0.min(c2.0);
                    let end = c1.0.max(c2.0);
                    for row in start.0..=end.0 {
                        add_rock(CaveCoords(CaveCoord(row), c1.1));
                    }
                } else {
                    panic!("diagonal segment?");
                }
            }
        }
    }

    let cave_extent = CaveExtent {
        greatest_row: CaveCoord(greatest_row),
        smallest_column: CaveCoord(smallest_column),
        greatest_column: CaveCoord(greatest_column),
    };

    Grid::new(&rocks, &cave_extent, floor)
}

#[test]
fn run_example() {
    const EXAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    // Part 1.
    {
        let mut grid = parse_to_grid(EXAMPLE, Floor::None);
        let grains_count = grid.add_all_sand();
        assert_eq!(grains_count, 24);
    }

    // Part 2.
    {
        let mut grid = parse_to_grid(EXAMPLE, Floor::Infinite);
        let grains_count = grid.add_all_sand();
        assert_eq!(grains_count, 93);
    }
}

fn main() {
    let input = include_str!("../input");

    // Part 1.
    {
        let mut grid = parse_to_grid(input, Floor::None);
        let grains_count = grid.add_all_sand();
        println!("grains added, no floor: {}", grains_count);
        assert_eq!(grains_count, 715);
    }

    // Part 2.
    {
        let mut grid = parse_to_grid(input, Floor::Infinite);
        let grains_count = grid.add_all_sand();
        println!("grains added, infinite floor: {}", grains_count);
        assert_eq!(grains_count, 25248);
    }
}
