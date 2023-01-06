use itertools::Itertools;
use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Int(i32),
}

#[derive(PartialEq, Eq, Debug)]
enum Token {
    Open,
    Close,
    Comma,
    Integer(i32),
}

struct Tokens<'a> {
    stream: Peekable<Chars<'a>>,
}

impl<'a> Tokens<'a> {
    fn new(s: &'a str) -> Tokens<'a> {
        Tokens {
            stream: s.chars().peekable(),
        }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match *self.stream.peek()? {
                '[' => {
                    self.stream.next();
                    return Some(Token::Open);
                }
                ']' => {
                    self.stream.next();
                    return Some(Token::Close);
                }
                ',' => {
                    self.stream.next();
                    return Some(Token::Comma);
                }
                c => {
                    if c.is_whitespace() {
                        self.stream.next();
                        continue;
                    }

                    assert!(c.is_digit(10));

                    let digits = self
                        .stream
                        .take_while_ref(|c| c.is_digit(10))
                        .collect::<String>();

                    return Some(Token::Integer(digits.parse().expect("integer")));
                }
            }
        }
    }
}

#[test]
fn test_tokenizing() {
    assert_eq!(
        Tokens::new("[[8,[1,9],6]]").into_iter().collect::<Vec<_>>(),
        [
            Token::Open,
            Token::Open,
            Token::Integer(8),
            Token::Comma,
            Token::Open,
            Token::Integer(1),
            Token::Comma,
            Token::Integer(9),
            Token::Close,
            Token::Comma,
            Token::Integer(6),
            Token::Close,
            Token::Close
        ]
    );
}

fn parse_list_contents(tokens: &mut Tokens) -> Vec<Packet> {
    let mut elems = vec![];

    let mut tok = tokens.next().expect("list contents");
    if let Token::Close = tok {
        return elems;
    }

    loop {
        if let Token::Integer(i) = tok {
            elems.push(Packet::Int(i));
        } else if let Token::Open = tok {
            elems.push(Packet::List(parse_list_contents(tokens)));
        } else {
            panic!("expected list element: {:?}", tok);
        }

        match tokens.next().expect("after element") {
            Token::Close => break,
            Token::Comma => {
                tok = tokens.next().expect("next element");
                continue;
            }
            tok => panic!("unexpected token: {:?}", tok),
        }
    }

    elems
}

fn parse_packet(s: &str) -> Packet {
    let mut tokens = Tokens::new(s);

    if let Token::Open = tokens.next().expect("token") {
        return Packet::List(parse_list_contents(&mut tokens));
    } else {
        panic!("didn't get integer or list");
    }
}

fn compare_packets(p1: &Packet, p2: &Packet) -> Ordering {
    match (p1, p2) {
        (&Packet::Int(lint), &Packet::Int(rint)) => lint.cmp(&rint),
        (&Packet::List(ref llist), &Packet::List(ref rlist)) => {
            let mut lelems = llist.iter();
            let mut relems = rlist.iter();

            loop {
                let left = lelems.next();
                let right = relems.next();

                match (left, right) {
                    (None, Some(_)) => return Ordering::Less,
                    (Some(_), None) => return Ordering::Greater,
                    (None, None) => return Ordering::Equal,
                    (Some(left), Some(right)) => match compare_packets(left, right) {
                        Ordering::Equal => continue,
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                    },
                }
            }
        }
        (&Packet::Int(lint), right_packet_list) => {
            compare_packets(&Packet::List(vec![Packet::Int(lint)]), right_packet_list)
        }
        (left_packet_list, &Packet::Int(rint)) => {
            compare_packets(left_packet_list, &Packet::List(vec![Packet::Int(rint)]))
        }
    }
}

#[test]
fn test_comparison() {
    macro_rules! compare {
        ($first:expr, $second:expr, $cmp:expr) => {
            let first = parse_packet($first);
            let second = parse_packet($second);
            assert_eq!(compare_packets(&first, &second), $cmp);
        };
    }

    compare!("[]", "[]", Ordering::Equal);
    compare!("[]", "[1]", Ordering::Less);
    compare!("[1]", "[]", Ordering::Greater);
    compare!("[1]", "[[1]]", Ordering::Equal);
    compare!("[1, [2], 3]", "[1, 2, 3]", Ordering::Equal);
}

#[test]
fn test_example() {
    let example = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    let pairs = parse_input(example);

    let sum_right_ordered_pairs_indices = Itertools::tuples(pairs.iter())
        .map(|(left, right)| {
            let res = compare_packets(left, right);
            res
        })
        .enumerate()
        .filter_map(|(i, order)| match order {
            Ordering::Less => {
                assert!([1, 2, 4, 6].contains(&(i + 1)), "i should be 1/2/4/6");
                Some(i + 1)
            }
            _ => {
                assert!(![1, 2, 4, 6].contains(&(i + 1)), "i shouldn't be 1/2/4/6");
                None
            }
        })
        .sum::<usize>();

    assert_eq!(sum_right_ordered_pairs_indices, 13);

    let div1 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let div2 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);

    let mut pairs_and_dividers = vec![div1.clone(), div2.clone()];
    pairs_and_dividers.extend(pairs);

    pairs_and_dividers.sort_by(compare_packets);

    assert!(pairs_and_dividers.contains(&div1));
    assert!(pairs_and_dividers.contains(&div2));

    let index_div1 = pairs_and_dividers
        .binary_search_by(|packet| compare_packets(packet, &div1))
        .ok()
        .expect("div1")
        + 1;
    assert_eq!(index_div1 + 1, 10);

    println!(
        "contains index_div2: {}",
        pairs_and_dividers.contains(&div2)
    );
    if false {
        let index_div2 =
            pairs_and_dividers.binary_search_by(|packet| compare_packets(packet, &div2));
        println!("index_div2: {:?}", index_div2);

        let index_div2 = index_div2.ok().expect("div2") + 1;
        println!("index of [[6]]: {}", index_div2);

        println!(
            "indexes of dividers after sorting: {}, {}",
            index_div1, index_div2
        );

        println!("product of indexes: {}", index_div1 * index_div2);
    }
}

fn parse_input(s: &str) -> Vec<Packet> {
    s.lines()
        .filter(|s| !s.is_empty())
        .map(parse_packet)
        .collect()
}

fn main() {
    let pairs = parse_input(include_str!("../input"));

    let sum_right_ordered_pairs_indices = Itertools::tuples(pairs.iter())
        .map(|(left, right)| {
            let res = compare_packets(left, right);
            res
        })
        .enumerate()
        .filter_map(|(i, order)| match order {
            Ordering::Less => Some(i + 1),
            _ => None,
        })
        .sum::<usize>();
    println!(
        "sum of indexes of right-ordered pairs: {}",
        sum_right_ordered_pairs_indices
    );
    assert_eq!(sum_right_ordered_pairs_indices, 5013);

    let div1 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let div2 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);

    let mut pairs_and_dividers = vec![div1.clone(), div2.clone()];
    pairs_and_dividers.extend(pairs);

    pairs_and_dividers.sort_by(compare_packets);

    let index_div1 = pairs_and_dividers
        .binary_search_by(|packet| compare_packets(packet, &div1))
        .ok()
        .expect("div1")
        + 1;
    println!("index of [[2]]: {}", index_div1);

    let index_div2 = pairs_and_dividers
        .binary_search_by(|packet| compare_packets(packet, &div2))
        .ok()
        .expect("div2")
        + 1;
    println!("index of [[6]]: {:?}", index_div2);

    println!(
        "indexes of dividers after sorting: {}, {}",
        index_div1, index_div2
    );

    let prod = index_div1 * index_div2;
    println!("product of indexes: {}", prod);
    assert_eq!(prod, 25038);
}
