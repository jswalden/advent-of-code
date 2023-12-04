use std::collections::HashSet;

struct Card {
    winning_numbers: HashSet<u8>,
    have_numbers: HashSet<u8>,
}

impl Card {
    fn new(s: &str) -> Card {
        let (_, rest) = s.split_once(": ").expect("colon");

        let (winning, have) = rest.split_once(" | ").expect("pipe");

        fn parse_numbers(s: &str) -> HashSet<u8> {
            s.trim()
                .split_ascii_whitespace()
                .map(|s| s.parse::<u8>().expect("winparse"))
                .collect()
        }

        let winning_numbers = parse_numbers(winning);
        let have_numbers = parse_numbers(have);

        Card {
            winning_numbers,
            have_numbers,
        }
    }

    fn matching_numbers(&self) -> usize {
        self.winning_numbers
            .intersection(&self.have_numbers)
            .count()
    }
}

#[test]
fn example() {
    static INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let cards: Vec<_> = INPUT.lines().map(Card::new).collect();
    let card_original_matches: Vec<_> = cards.iter().map(Card::matching_numbers).collect();

    // Part 1.
    println!("Part 1");
    let pile_worth: u32 = card_original_matches
        .iter()
        .copied()
        .map(|n| if n == 0 { 0 } else { 2_u32.pow(n as u32 - 1) })
        .sum();
    println!("Pile worth: {pile_worth}");
    assert_eq!(pile_worth, 13);

    // Part 2.
    println!("Part 2:");

    let mut card_copies: Vec<_> = cards.iter().map(|_| 1).collect();

    for (i, value) in card_original_matches.iter().enumerate() {
        for j in 0..*value {
            let cm = i + 1 + j as usize;
            let amt = card_copies[i];
            card_copies[cm] += amt;
        }
    }

    let total_cards: usize = card_copies.iter().sum();
    println!("Total cards: {total_cards}");
    assert_eq!(total_cards, 30);
}

fn main() {
    static INPUT: &str = include_str!("../input");

    let cards: Vec<_> = INPUT.lines().map(Card::new).collect();
    let card_original_matches: Vec<_> = cards.iter().map(Card::matching_numbers).collect();

    // Part 1.
    println!("Part 1");
    let pile_worth: u32 = card_original_matches
        .iter()
        .copied()
        .map(|n| if n == 0 { 0 } else { 2_u32.pow(n as u32 - 1) })
        .sum();
    println!("Pile worth: {pile_worth}");
    assert_eq!(pile_worth, 25_174);

    // Part 2.
    println!("Part 2:");

    let mut card_copies: Vec<_> = cards.iter().map(|_| 1).collect();

    for (i, value) in card_original_matches.iter().enumerate() {
        for j in 0..*value {
            let cm = i + 1 + j as usize;
            let amt = card_copies[i];
            card_copies[cm] += amt;
        }
    }

    let total_cards: usize = card_copies.iter().sum();
    println!("Total cards: {total_cards}");
    assert_eq!(total_cards, 6_420_979);
}
