use std::str::Lines;

struct GameIter<'a> {
    lines: Lines<'a>,
}

impl<'a> GameIter<'a> {
    fn new(input: &'a str) -> GameIter<'a> {
        GameIter {
            lines: input.lines(),
        }
    }
}

struct RequiredAmount {
    game_num: usize,
    red: usize,
    green: usize,
    blue: usize,
}

impl<'a> Iterator for GameIter<'a> {
    type Item = RequiredAmount;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next()?;

        let game_start = "Game ".len();
        let game_end = game_start + line[game_start..].find(':').expect("colon");
        let game_num = line[game_start..game_end]
            .parse::<usize>()
            .expect("game_num");

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for game in line[game_end + 1 + 1..].split("; ") {
            for color_count in game.split(", ") {
                let num_end = color_count.find(' ').expect("end of count");
                let num = color_count[0..num_end].parse::<usize>().expect("num");

                match &color_count[num_end + 1..] {
                    "red" => red = red.max(num),
                    "green" => green = green.max(num),
                    "blue" => blue = blue.max(num),
                    _ => panic!("bad color"),
                }
            }
        }

        Some(RequiredAmount {
            game_num,
            red,
            green,
            blue,
        })
    }
}

#[test]
fn example() {
    static INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    // Part 1.
    println!("Part 1:");
    let sum_possible_games = GameIter::new(&INPUT)
        .filter_map(|gi| {
            if gi.red <= 12 && gi.green <= 13 && gi.blue <= 14 {
                Some(gi.game_num)
            } else {
                None
            }
        })
        .sum::<usize>();
    println!("Sum: {sum_possible_games}");
    assert_eq!(sum_possible_games, 8);

    // Part 2.
    println!("Part 2:");
    let sum_of_powers = GameIter::new(&INPUT)
        .map(|gi| gi.red * gi.green * gi.blue)
        .sum::<usize>();
    println!("Sum of powers: {sum_of_powers}");
    assert_eq!(sum_of_powers, 2_286);
}

fn main() {
    static INPUT: &str = include_str!("../input");

    // Part 1.
    println!("Part 1:");
    let sum_possible_games = GameIter::new(&INPUT)
        .filter_map(|gi| {
            if gi.red <= 12 && gi.green <= 13 && gi.blue <= 14 {
                Some(gi.game_num)
            } else {
                None
            }
        })
        .sum::<usize>();
    println!("Sum: {sum_possible_games}");
    assert_eq!(sum_possible_games, 2_439);

    // Part 2.
    println!("Part 2:");
    let sum_of_powers = GameIter::new(&INPUT)
        .map(|gi| gi.red * gi.green * gi.blue)
        .sum::<usize>();
    println!("Sum of powers: {sum_of_powers}");
    assert_eq!(sum_of_powers, 63_711);
}
