static CONTENT: &str = include_str!("../input");

fn create_piles() -> Vec<Vec<char>> {
    let mut piles = vec![vec![]; 9];

    static PILES: &'static [&'static str] = &[
        "NSDCVQT", "MFV", "FQWDPNHM", "DQRTF", "RFMNQHVB", "CFGNPWQ", "WFRLCT", "TZNS", "MSDJRQHN",
    ];

    for (i, pile) in PILES.iter().enumerate() {
        piles[i].extend((*pile).chars());
    }

    piles
}

#[derive(Copy, Clone)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_moves() -> Vec<Move> {
    let mut moves = vec![];

    for line in CONTENT.lines() {
        let mut splits = line.split(' ');

        macro_rules! match_token {
            ($token:literal) => {
                match splits.next() {
                    None => false,
                    Some(tok) => tok.eq($token),
                }
            };
        }

        match splits.next() {
            None => continue,
            Some(tok) => {
                if !tok.eq("move") {
                    continue;
                }
            }
        }

        let count = splits.next().expect("count").parse().expect("count");
        match_token!("from");

        let from = splits.next().expect("from").parse().expect("from");
        match_token!("to");

        let to = splits.next().expect("to").parse().expect("to");

        if !splits.next().is_none() {
            continue;
        }

        moves.push(Move { count, from, to });
    }

    moves
}

enum Part {
    Part1,
    Part2,
}

fn part(mut piles: Vec<Vec<char>>, moves: Vec<Move>, part: Part) {
    for Move { count, from, to } in moves {
        match part {
            Part::Part1 => {
                for _ in 1..=count {
                    let removed = piles[from - 1].pop().expect("from pile");
                    piles[to - 1].push(removed);
                }
            }
            Part::Part2 => {
                let from = &mut piles[from - 1];
                let removed = from.split_off(from.len() - count);
                piles[to - 1].extend(removed);
            }
        }
    }

    let tops = piles
        .into_iter()
        .map(|pile| pile.last().expect("empty pile").clone())
        .fold(String::new(), |mut acc, c| {
            acc.push(c);
            acc
        });
    println!("Part 1 tops of piles: {}", tops);
}

fn main() {
    let piles = create_piles();

    let moves = parse_moves();

    part(piles.clone(), moves.clone(), Part::Part1);
    part(piles.clone(), moves.clone(), Part::Part2);
}
