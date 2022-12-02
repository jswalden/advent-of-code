static CONTENTS: &str = include_str!("../input");

#[derive(Copy, Clone, PartialEq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

fn shape_score(play: Play) -> u64 {
    match play {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissors => 3,
    }
}

fn opponent_play(s: &str) -> Play {
    match s {
        "A" => Play::Rock,
        "B" => Play::Paper,
        "C" => Play::Scissors,
        _ => panic!("unexpected opponent play: {}", s),
    }
}

fn your_play(s: &str) -> Play {
    match s {
        "X" => Play::Rock,
        "Y" => Play::Paper,
        "Z" => Play::Scissors,
        _ => panic!("unexpected your play: {}", s),
    }
}

fn outcome_score(op: Play, yp: Play) -> u64 {
    if op == yp {
        3
    } else if ((op as u8 + 1) % 3) == yp as u8 {
        6
    } else {
        0
    }
}

fn round_score(op: Play, yp: Play) -> u64 {
    fn display(p: Play) -> &'static str {
        match p {
            Play::Rock => "Rock",
            Play::Paper => "Paper",
            Play::Scissors => "Scissors",
        }
    }

    let ss = shape_score(yp);
    let os = outcome_score(op, yp);

    print!(
        "{}-{}: shape {}, outcome {}\n",
        display(op),
        display(yp),
        ss,
        os
    );
    ss + os
}

fn part1() {
    let score = CONTENTS
        .lines()
        .into_iter()
        .map(|line| {
            let plays = line.split(' ').collect::<Vec<_>>();

            round_score(opponent_play(plays[0]), your_play(plays[1]))
        })
        .fold(0, |acc, score| acc + score);
    print!("Part 1 score: {}\n", score);
}

fn to_play(p: u8) -> Play {
    match p {
        0 => Play::Rock,
        1 => Play::Paper,
        2 => Play::Scissors,
        _ => panic!("unexpected opponent play value: {}", p),
    }
}

fn part2() {
    let score = CONTENTS
        .lines()
        .into_iter()
        .map(|line| {
            let plays = line.split(' ').collect::<Vec<_>>();

            let op = opponent_play(plays[0]);

            let opn = op as u8;
            let yp = to_play(match plays[1] {
                "X" => (opn + 2) % 3,
                "Y" => opn,
                "Z" => (opn + 1) % 3,
                _ => panic!("unexpected your play: {}", plays[1]),
            });

            print!("{}: ", line);
            let s = round_score(op, yp);
            s
        })
        .fold(0, |acc, score| acc + score);
    print!("Part 2 score: {}\n", score);
}

fn main() {
    part1();
    part2();
}
