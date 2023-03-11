use std::collections::{HashMap, HashSet};

type Monkey = String;

enum YellType {
    Add,
    Sub,
    Div,
    Mul,
}
struct YellEquation {
    yell_type: YellType,
    args: (Monkey, Monkey),
}

impl YellEquation {
    fn evaluate(&self, resolved: &HashMap<Monkey, MonkeyNumber>) -> Option<MonkeyNumber> {
        let x = *(resolved.get(&self.args.0)?);
        let y = *(resolved.get(&self.args.1)?);
        Some(match self.yell_type {
            YellType::Add => x + y,
            YellType::Sub => x - y,
            YellType::Div => x / y,
            YellType::Mul => x * y,
        })
    }
}

enum Yell {
    Number(MonkeyNumber),
    Equation(YellEquation),
}

type MonkeyNumber = i64;

struct MonkeyYell {
    name: Monkey,
    yell: Yell,
}

fn parse_monkey_yell(s: &str) -> MonkeyYell {
    let colon = s.find(':').expect("colon");

    // nsdv: czts * nlpw
    // mrgj: 3
    let monkey_name = s[0..colon].to_string();

    let rest = &s[colon + 2..];
    MonkeyYell {
        name: monkey_name,
        yell: if let Ok(n) = rest.parse() {
            Yell::Number(n)
        } else {
            let space = rest.find(' ').expect("first space");
            Yell::Equation(YellEquation {
                yell_type: match &rest[space + 1..space + 2] {
                    "+" => YellType::Add,
                    "-" => YellType::Sub,
                    "/" => YellType::Div,
                    "*" => YellType::Mul,
                    other => panic!("unexpected yell: {other}"),
                },
                args: (rest[0..space].to_string(), rest[space + 3..].to_string()),
            })
        },
    }
}

struct MonkeyGraph {
    resolved: HashMap<Monkey, MonkeyNumber>,
}

enum Part {
    Part1,
    Part2,
}

impl MonkeyGraph {
    fn new(input: &str, part: Part) -> MonkeyGraph {
        struct PartialGraph {
            resolved: HashMap<Monkey, MonkeyNumber>,
            unresolved: HashMap<Monkey, YellEquation>,
            deps: HashMap<Monkey, HashSet<Monkey>>,
            part: Part,
        }
        impl PartialGraph {
            fn new(part: Part) -> PartialGraph {
                PartialGraph {
                    resolved: HashMap::new(),
                    unresolved: HashMap::new(),
                    deps: HashMap::new(),
                    part,
                }
            }

            fn process_yelled_number(&mut self, monkey_name: Monkey, number: MonkeyNumber) {
                self.resolved.insert(monkey_name.clone(), number);

                if let Some(waiting_monkeys) = self.deps.remove(&monkey_name) {
                    for waiting_monkey_name in waiting_monkeys {
                        let eqn = self
                            .unresolved
                            .get(&waiting_monkey_name)
                            .expect("unresolved");

                        if let Some(n) = eqn.evaluate(&self.resolved) {
                            self.process_yelled_number(waiting_monkey_name, n);
                        }
                    }
                }
            }

            fn add_dependency(&mut self, from: Monkey, to: &Monkey) {
                if self.resolved.contains_key(to) {
                    return;
                }

                self.deps
                    .entry(to.clone())
                    .or_insert(HashSet::new())
                    .insert(from);
            }

            fn process_yelled_equation(&mut self, monkey_name: Monkey, eqn: YellEquation) {
                self.add_dependency(monkey_name.clone(), &eqn.args.0);
                self.add_dependency(monkey_name.clone(), &eqn.args.1);

                if let Some(n) = eqn.evaluate(&self.resolved) {
                    self.process_yelled_number(monkey_name, n);
                } else {
                    self.unresolved.insert(monkey_name, eqn);
                }
            }

            fn process_monkey_yell(&mut self, monkey_name: Monkey, yell: Yell) {
                match yell {
                    Yell::Number(number) => self.process_yelled_number(monkey_name, number),
                    Yell::Equation(eqn) => self.process_yelled_equation(monkey_name, eqn),
                }
            }
        }

        let mut graph = PartialGraph::new(part);

        for MonkeyYell { name, yell } in input.trim().lines().map(parse_monkey_yell) {
            graph.process_monkey_yell(name, yell);
        }

        assert!(graph.deps.is_empty());
        // graph.unresolved still has entries in it.

        MonkeyGraph {
            resolved: graph.resolved,
        }
    }

    fn monkey_number(&self, monkey: Monkey) -> MonkeyNumber {
        *self.resolved.get(&monkey).expect("monkey name")
    }
}

fn root_monkey_yells(input: &str, part: Part) -> MonkeyNumber {
    MonkeyGraph::new(input, part).monkey_number("root".to_string())
}

#[test]
fn example() {
    static INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    let root_yells = root_monkey_yells(INPUT);
    println!("Part 1 root yells: {root_yells}");
    assert_eq!(root_yells, 152)
}

fn main() {
    static INPUT: &str = include_str!("../input");

    // Part 1.
    let root_yells = root_monkey_yells(INPUT);
    println!("Part 1 root yells: {root_yells}");
    assert_eq!(root_yells, 75_147_370_123_646);

    // Part 2.
}
