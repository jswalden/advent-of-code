struct Grid {
    pub data: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    fn new(s: &str) -> Grid {
        let mut data = vec![];
        let mut height = 0;
        let mut width = None;
        for line in s.lines() {
            data.push(line.chars().collect());
            width = Some(data.len());
            height += 1;
        }

        Grid {
            data,
            height,
            width: width.expect("width"),
        }
    }

    fn symbols(&self) -> SymbolsIter {
        SymbolsIter::new(self)
    }
}

struct SymbolsIter<'a> {
    grid: &'a Grid,
    next_index: usize,
}

impl<'a> SymbolsIter<'a> {
    fn new(grid: &'a Grid) -> SymbolsIter {
        SymbolsIter {
            grid,
            next_index: 0,
        }
    }
}

impl<'a> Iterator for SymbolsIter<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let idx = self.next_index;
            if idx >= self.grid.width * self.grid.height {
                return None;
            }

            let (x, y) = (idx % self.grid.width, idx / self.grid.width);
            self.next_index += 1;
            match self.grid.data[y][x] {
                '0'..='9' | '.' => {}
                _ => {
                    return Some((y, x));
                }
            }
        }
    }
}

fn is_symbol(c: char) -> bool {
    match c {
        '0'..='9' | '.' => false,
        _ => true,
    }
}

fn sum_numbers_by_symbols(grid: &Grid) -> u32 {
    let mut sum = 0;

    for y in 0..grid.height {
        let mut x = 0;
        while x < grid.width {
            match grid.data[y][x] {
                '.' => {}
                '0'..='9' => {
                    let begin = x;
                    while x < grid.width && grid.data[y][x].is_ascii_digit() {
                        x += 1;
                    }

                    let end = x - 1;

                    let before_x = begin.saturating_sub(1);
                    let after_x = (end + 1).min(grid.width - 1);

                    if (y > 0 && (before_x..=after_x).any(|x| is_symbol(grid.data[y - 1][x])))
                        || (y < grid.height - 1
                            && (before_x..=after_x).any(|x| is_symbol(grid.data[y + 1][x])))
                        || (begin > 0 && is_symbol(grid.data[y][begin - 1]))
                        || (end < grid.width - 1 && is_symbol(grid.data[y][end + 1]))
                    {
                        let mut n = 0;
                        for x in begin..=end {
                            n *= 10;
                            n += grid.data[y][x].to_digit(10).expect("digit");
                        }

                        sum += n;
                        continue;
                    }
                }
                _ => {}
            }

            x += 1;
        }
    }

    sum
}

fn number_at(grid: &Grid, y: usize, x: usize) -> u32 {
    let mut start = x;
    assert!(grid.data[y][x].is_ascii_digit());

    while start > 0 && grid.data[y][start - 1].is_ascii_digit() {
        start -= 1;
    }
    let mut end = x;
    while end < grid.width - 1 && grid.data[y][end + 1].is_ascii_digit() {
        end += 1;
    }

    let mut n = 0;
    for x in start..=end {
        n *= 10;
        n += grid.data[y][x].to_digit(10).expect("digit");
    }

    n
}

fn sum_gear_ratios(grid: &Grid) -> u32 {
    let mut sum = 0;
    for (y, x) in grid.symbols() {
        if grid.data[y][x] != '*' {
            continue;
        }

        let mut number_positions = vec![];

        if y > 0 {
            let above_left = (x > 0 && grid.data[y - 1][x - 1].is_ascii_digit()) as usize;
            let above = grid.data[y - 1][x].is_ascii_digit() as usize;
            let above_right =
                (x < grid.width - 1 && grid.data[y - 1][x + 1].is_ascii_digit()) as usize;

            let sum = above_left + above + above_right;
            if sum == 0 {
                // Do nothing.
            } else if sum == 2 {
                if above == 0 {
                    number_positions.push((y - 1, x - 1));
                    number_positions.push((y - 1, x + 1));
                } else {
                    number_positions.push((y - 1, x));
                }
            } else if sum == 3 || above == 1 {
                number_positions.push((y - 1, x));
            } else if above_left == 1 {
                number_positions.push((y - 1, x - 1));
            } else {
                assert_eq!(above_right, 1);
                number_positions.push((y - 1, x + 1));
            }
        }

        if x > 0 && grid.data[y][x - 1].is_ascii_digit() {
            number_positions.push((y, x - 1));
        }
        if x < grid.width - 1 && grid.data[y][x + 1].is_ascii_digit() {
            number_positions.push((y, x + 1));
        }

        if y < grid.height - 1 {
            let below_left = (x > 0 && grid.data[y + 1][x - 1].is_ascii_digit()) as usize;
            let below = grid.data[y + 1][x].is_ascii_digit() as usize;
            let below_right =
                (x < grid.width - 1 && grid.data[y + 1][x + 1].is_ascii_digit()) as usize;

            let sum = below_left + below + below_right;
            if sum == 0 {
                // Do nothing.
            } else if sum == 2 {
                if below == 0 {
                    number_positions.push((y + 1, x - 1));
                    number_positions.push((y + 1, x + 1));
                } else {
                    number_positions.push((y + 1, x));
                }
            } else if sum == 3 || below == 1 {
                number_positions.push((y + 1, x));
            } else if below_left == 1 {
                number_positions.push((y + 1, x - 1));
            } else {
                assert_eq!(below_right, 1);
                number_positions.push((y + 1, x + 1));
            }
        }

        if number_positions.len() == 2 {
            sum += number_positions
                .iter()
                .fold(1, |acc, (y, x)| acc * number_at(grid, *y, *x));
        }
    }

    sum
}

#[test]
fn example() {
    static INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    let grid = Grid::new(&INPUT);

    // Part 1.
    let sum = sum_numbers_by_symbols(&grid);
    println!("Sum of numbers by symbols: {sum}");
    assert_eq!(sum, 4361);

    // Part 2.
    let sum_gear_ratios = sum_gear_ratios(&grid);
    println!("Sum of numbers by gears: {sum_gear_ratios}");
    assert_eq!(sum_gear_ratios, 467_835);
}

fn main() {
    static INPUT: &str = include_str!("../input");

    let grid = Grid::new(&INPUT);

    // Part 1.
    let sum = sum_numbers_by_symbols(&grid);
    println!("Sum of numbers by symbols: {sum}");
    assert_eq!(sum, 531_932);

    // Part 2.
    let sum_gear_ratios = sum_gear_ratios(&grid);
    println!("Sum of numbers by gears: {sum_gear_ratios}");
    assert_eq!(sum_gear_ratios, 73_646_890);
}
