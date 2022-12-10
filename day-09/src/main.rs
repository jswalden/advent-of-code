use std::collections::HashSet;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Pos(i32, i32);

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

struct Delta(i32, i32);

impl Direction {
    fn delta(&self) -> Delta {
        match *self {
            Direction::Up => Delta(0, 1),
            Direction::Left => Delta(-1, 0),
            Direction::Right => Delta(1, 0),
            Direction::Down => Delta(0, -1),
        }
    }
}

impl Pos {
    fn adjust(&self, direction: &Direction) -> Pos {
        let Delta(x, y) = direction.delta();
        self.adjust_by(x, y)
    }

    fn adjust_by(&self, x: i32, y: i32) -> Pos {
        let mut fresh = *self;
        fresh.0 += x;
        fresh.1 += y;
        fresh
    }
}

struct Move {
    direction: Direction,
    count: i32,
}

fn parse_moves(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split(' ');
            Move {
                direction: match it.next().expect("direction") {
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    s => panic!("bad direction: {}", s),
                },
                count: it.next().expect("<count>").parse().expect("count"),
            }
        })
        .collect()
}

fn make_rope(rope_len: usize) -> (Vec<Pos>, HashSet<Pos>) {
    assert!(rope_len > 1, "rope must be at least length two");

    let rope = vec![Pos(0, 0); rope_len];
    let mut tail_position_set = HashSet::new();
    tail_position_set.insert(rope[0]);

    (rope, tail_position_set)
}

#[must_use]
fn move_tail(new_head_pos: &Pos, tail_pos: &mut Pos) -> bool {
    if new_head_pos.0 == tail_pos.0 {
        // If head and tail are in a column...
        if (new_head_pos.1 - tail_pos.1).abs() < 2 {
            // Tail is adjacent to new head position (or beneath it) so doesn't
            // move.
            false
        } else {
            // Position in the same column, horizontally halfway between old
            // tail and new head.
            *tail_pos = Pos(tail_pos.0, (new_head_pos.1 + tail_pos.1) / 2);
            true
        }
    } else if new_head_pos.1 == tail_pos.1 {
        // If head and tail are in a row...
        if (new_head_pos.0 - tail_pos.0).abs() < 2 {
            // Tail is adjacent to new head position so doesn't move.
            false
        } else {
            // Position in the same row, horizontally halfway between old tail
            // and new head.
            *tail_pos = Pos((new_head_pos.0 + tail_pos.0) / 2, tail_pos.1);
            true
        }
    } else if (new_head_pos.0 - tail_pos.0).abs() == 2 && (new_head_pos.1 - tail_pos.1).abs() == 1 {
        // "Horizontal" rook move.
        *tail_pos = Pos((new_head_pos.0 + tail_pos.0) / 2, new_head_pos.1);
        true
    } else if (new_head_pos.1 - tail_pos.1).abs() == 2 && (new_head_pos.0 - tail_pos.0).abs() == 1 {
        // "Vertical" rook move.
        *tail_pos = Pos(new_head_pos.0, (new_head_pos.1 + tail_pos.1) / 2);
        true
    } else if (new_head_pos.0 - tail_pos.0).abs() == 1 && (new_head_pos.1 - tail_pos.1).abs() == 1 {
        // Diagonal and adjacent.  Tail does not move.
        false
    } else {
        assert!(
            (new_head_pos.0 - tail_pos.0).abs() == 2 && (new_head_pos.1 - tail_pos.1).abs() == 2,
            "previously-diagonal positions, head moved diagonally further out"
        );
        *tail_pos = Pos(
            (new_head_pos.0 + tail_pos.0) / 2,
            (new_head_pos.1 + tail_pos.1) / 2,
        );
        true
    }
}

fn count_tail_positions(
    moves: &Vec<Move>,
    rope: &mut Vec<Pos>,
    tail_position_set: &mut HashSet<Pos>,
) {
    let rope_len = rope.len();
    assert!(rope_len > 1, "rope must have at least a head and tail");

    // rope.first() is head, rope.last() is tail.
    for Move { direction, count } in moves {
        for _ in 0..*count {
            // Move the head.
            rope[0] = rope[0].adjust(direction);

            // Move all tails.
            for tail_start in 1..rope_len {
                let (heads, tails) = rope.split_at_mut(tail_start);
                if !move_tail(&heads[tail_start - 1], &mut tails[0]) {
                    break;
                }
            }

            let tail_pos = rope[rope_len - 1];
            //println!("Tail moves to ({}, {})", tail_pos.0, tail_pos.1);
            tail_position_set.insert(tail_pos);
        }
    }
}

#[cfg(test)]
fn run_move_test(expected_count: usize, rope_len: usize, input: &str) {
    let moves = parse_moves(input);
    let (mut rope, mut tail_position_set) = make_rope(rope_len);
    count_tail_positions(&moves, &mut rope, &mut tail_position_set);
    let tpc = tail_position_set.len();
    assert!(tpc == expected_count);
}

#[test]
fn test_no_move() {
    run_move_test(1, 2, "R 0");
}

#[test]
fn test_one_move() {
    run_move_test(1, 2, "R 1");
}

#[test]
fn test_back_and_forth() {
    run_move_test(
        1,
        2,
        "U 1
D 2
U 1",
    );
}

#[test]
fn test_example_length2() {
    run_move_test(
        13,
        2,
        "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
    );
}

#[test]
fn test_example_length10() {
    run_move_test(
        36,
        10,
        "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
    );
}

fn main() {
    let moves = parse_moves(include_str!("../input"));

    // Part 1.
    let (mut rope_two, mut tail_position_set) = make_rope(2);
    count_tail_positions(&moves, &mut rope_two, &mut tail_position_set);

    let tail_pos_count = tail_position_set.len();
    println!("Tail visits {} positions", tail_pos_count);
    assert!(tail_pos_count == 5878);

    // Part 2.
    let (mut rope_ten, mut tail_position_set) = make_rope(10);
    count_tail_positions(&moves, &mut rope_ten, &mut tail_position_set);
    let tail_pos_count = tail_position_set.len();
    println!("Tail visits {} positions", tail_pos_count);
    assert!(tail_pos_count == 2405);
}
