use itertools::Itertools;
use std::cmp::Ordering;

const JACK: char = 'J';

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(Copy, Clone)]
enum Part {
    One,
    Two,
}

fn card_value(c: char, part: Part) -> u8 {
    match c {
        c @ '2'..='9' => c.to_digit(10).expect("digit") as u8,
        'T' => 10,
        JACK => match part {
            Part::One => 11,
            Part::Two => 1,
        },
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        c => panic!("unexpected card {c}"),
    }
}

fn hand_type(hand: &'static str, part: Part) -> HandType {
    let mut sorted = hand.chars().collect::<Vec<_>>();
    sorted.sort_by_key(|c| card_value(*c, part));

    let mut jacks_count = 0;
    let mut i = 0;
    let mut runs = vec![];
    match part {
        Part::One => {}
        Part::Two => {
            while i < sorted.len() && sorted[i] == JACK {
                jacks_count += 1;
                i += 1;
            }
        }
    }

    if i < sorted.len() {
        runs.push((sorted[i], 1i8));
        i += 1;
        for c in &sorted[i..] {
            match part {
                Part::One => {}
                Part::Two => {
                    if *c == JACK {
                        jacks_count += 1;
                        continue;
                    }
                }
            }
            let last = runs.last_mut().expect("last");
            if *c == last.0 {
                last.1 += 1;
            } else {
                runs.push((*c, 1));
            }
        }
    }

    runs.sort_by_key(|&(_, count)| -count);

    let most = if runs.len() > 0 {
        runs[0].1
    } else {
        assert_eq!(jacks_count, hand.len());
        0
    };

    match most + jacks_count as i8 {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => {
            if runs[1].1 == 2 {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        }
        2 => {
            if runs[1].1 == 2 {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        }
        1 => HandType::HighCard,
        _ => panic!("card counting error"),
    }
}

#[test]
fn test_hand_type() {
    assert_eq!(hand_type("32T3K", Part::One), HandType::OnePair,);
    assert_eq!(hand_type("T55J5", Part::One), HandType::ThreeOfAKind,);
    assert_eq!(hand_type("KK677", Part::One), HandType::TwoPair,);
    assert_eq!(hand_type("KTJJT", Part::One), HandType::TwoPair,);
    assert_eq!(hand_type("QQQJA", Part::One), HandType::ThreeOfAKind,);

    assert_eq!(hand_type("32T3K", Part::Two), HandType::OnePair,);
    assert_eq!(hand_type("T55J5", Part::Two), HandType::FourOfAKind,);
    assert_eq!(hand_type("KK677", Part::Two), HandType::TwoPair,);
    assert_eq!(hand_type("KKJ77", Part::Two), HandType::FullHouse,);
    assert_eq!(hand_type("JKK77", Part::Two), HandType::FullHouse,);
    assert_eq!(hand_type("KTJJT", Part::Two), HandType::FourOfAKind,);
    assert_eq!(hand_type("QQQJA", Part::Two), HandType::FourOfAKind,);
    assert_eq!(hand_type("J2345", Part::Two), HandType::OnePair,);
    assert_eq!(hand_type("2J345", Part::Two), HandType::OnePair,);
    assert_eq!(hand_type("23J45", Part::Two), HandType::OnePair,);
    assert_eq!(hand_type("234J5", Part::Two), HandType::OnePair,);
    assert_eq!(hand_type("2345J", Part::Two), HandType::OnePair,);

    assert_eq!(hand_type("JJJJJ", Part::One), HandType::FiveOfAKind);
    assert_eq!(hand_type("JJJJJ", Part::One), HandType::FiveOfAKind);
    assert_eq!(hand_type("JJJJQ", Part::One), HandType::FourOfAKind);
    assert_eq!(hand_type("JJJJQ", Part::Two), HandType::FiveOfAKind);
    assert_eq!(hand_type("JJQJJ", Part::One), HandType::FourOfAKind);
    assert_eq!(hand_type("JJQJJ", Part::Two), HandType::FiveOfAKind);
}

fn hands_and_bids(s: &'static str) -> impl Iterator<Item = (&'static str, u32)> {
    s.lines().map(|s| {
        let (hand, bid) = s.split_once(' ').expect("split_space");
        (hand, bid.parse::<u32>().expect("bid"))
    })
}

fn compare_by_type_then_cards_in_order(h1: &'static str, h2: &'static str, part: Part) -> Ordering {
    match hand_type(h1, part).cmp(&hand_type(h2, part)) {
        Ordering::Equal => {
            for (c1, c2) in Itertools::zip_eq(h1.chars(), h2.chars()) {
                match card_value(c1, part).cmp(&card_value(c2, part)) {
                    Ordering::Equal => continue,
                    ordering => return ordering,
                }
            }

            panic!("unexpected equal hands?");
        }
        ordering => ordering,
    }
}

#[test]
fn comparisons() {
    macro_rules! compare {
        ($hand1:literal, $hand2:literal, $part:path, $expect:path) => {
            assert_eq!(
                compare_by_type_then_cards_in_order($hand1, $hand2, $part),
                $expect
            );

            assert_eq!(
                compare_by_type_then_cards_in_order($hand2, $hand1, $part),
                $expect.reverse()
            )
        };
    }

    compare!("2345J", "23455", Part::One, Ordering::Less);
    compare!("2345J", "23455", Part::Two, Ordering::Less);

    compare!("2345J", "23454", Part::One, Ordering::Less);
    compare!("2345J", "23454", Part::Two, Ordering::Less);

    compare!("23329", "2332J", Part::One, Ordering::Less);
    compare!("23329", "2332J", Part::Two, Ordering::Less);

    compare!("JKKK2", "QQQQ2", Part::One, Ordering::Less);
    compare!("JKKK2", "QQQQ2", Part::Two, Ordering::Less);

    compare!("KJKK2", "QQQQ2", Part::One, Ordering::Less);
    compare!("KJKK2", "QQQQ2", Part::Two, Ordering::Greater);

    compare!("22345", "23455", Part::One, Ordering::Less);
    compare!("22325", "23455", Part::One, Ordering::Greater);
    compare!("22345", "23455", Part::One, Ordering::Less);
    compare!("22345", "23455", Part::One, Ordering::Less);

    compare!("2JJ34", "29934", Part::One, Ordering::Greater);
    compare!("JJ324", "99324", Part::One, Ordering::Greater);
}

#[test]
fn example() {
    static INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    let original_hands_bids = hands_and_bids(INPUT).collect::<Vec<_>>();

    // Part 1.
    println!("Part 1:");
    let mut sorted_hands = original_hands_bids.clone();
    sorted_hands.sort_by(|(h1, _), (h2, _)| compare_by_type_then_cards_in_order(h1, h2, Part::One));

    let mut total_winnings = 0;
    for (i, (_, bid)) in sorted_hands.iter().enumerate() {
        let i = i + 1;
        println!("Bid {bid}, rank {i}", bid = *bid);
        total_winnings += *bid as usize * i;
    }
    println!("Total winnings: {total_winnings}");
    assert_eq!(total_winnings, 6440);

    // Part 2.
    println!("Part 2:");
    let mut sorted_hands = original_hands_bids.clone();
    sorted_hands.sort_by(|(h1, _), (h2, _)| compare_by_type_then_cards_in_order(h1, h2, Part::Two));

    let mut total_winnings = 0;
    for (i, (_, bid)) in sorted_hands.iter().enumerate() {
        let i = i + 1;
        println!("Bid {bid}, rank {i}", bid = *bid);
        total_winnings += *bid as usize * i;
    }
    println!("Total winnings: {total_winnings}");
    assert_eq!(total_winnings, 5905);
}

fn main() {
    static INPUT: &str = include_str!("../input");

    let original_hands_bids = hands_and_bids(INPUT).collect::<Vec<_>>();

    // Part 1.
    println!("Part 1:");
    let mut sorted_hands = original_hands_bids.clone();
    sorted_hands.sort_by(|(h1, _), (h2, _)| compare_by_type_then_cards_in_order(h1, h2, Part::One));

    let mut total_winnings = 0;
    for (i, (_, bid)) in sorted_hands.iter().enumerate() {
        let i = i + 1;
        println!("Bid {bid}, rank {i}", bid = *bid);
        total_winnings += *bid as usize * i;
    }
    println!("Total winnings: {total_winnings}");
    assert_eq!(total_winnings, 251_806_792);

    // Part 2.
    println!("Part 2:");
    let mut sorted_hands = original_hands_bids.clone();
    sorted_hands.sort_by(|(h1, _), (h2, _)| compare_by_type_then_cards_in_order(h1, h2, Part::Two));

    let mut total_winnings = 0;
    for (i, (_, bid)) in sorted_hands.iter().enumerate() {
        let i = i + 1;
        println!("Bid {bid}, rank {i}", bid = *bid);
        total_winnings += *bid as usize * i;
    }
    println!("Total winnings: {total_winnings}");
    assert_eq!(total_winnings, 252_113_488);
}
