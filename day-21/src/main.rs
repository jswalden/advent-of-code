use std::collections::{HashMap, HashSet};

type Monkey = String;
type MonkeyNumber = i64;

#[derive(PartialEq, Eq)]
enum YellType {
    Add,
    Sub,
    Div,
    Mul,
    Equ,
}
struct YellEquation {
    yell_type: YellType,
    args: (Monkey, Monkey),
}

impl YellEquation {
    fn is_equality(&self) -> bool {
        if let YellType::Equ = self.yell_type {
            true
        } else {
            false
        }
    }

    fn evaluate(&self, resolved: &HashMap<Monkey, MonkeyNumber>) -> Option<MonkeyNumber> {
        let x = *(resolved.get(&self.args.0)?);
        let y = *(resolved.get(&self.args.1)?);
        Some(match self.yell_type {
            YellType::Add => x + y,
            YellType::Sub => x - y,
            YellType::Div => x / y,
            YellType::Mul => x * y,
            YellType::Equ => return None,
        })
    }

    fn solve_for(
        &self,
        resolved: &HashMap<Monkey, MonkeyNumber>,
        value: MonkeyNumber,
    ) -> (MonkeyNumber, Monkey) {
        match (resolved.get(&self.args.0), resolved.get(&self.args.1)) {
            (Some(x), None) => {
                let unknown_monkey = self.args.1.clone();

                let value = match self.yell_type {
                    YellType::Add => value - *x,
                    YellType::Sub => *x - value,
                    YellType::Div => *x / value,
                    YellType::Mul => value / *x,
                    YellType::Equ => panic!("shouldn't see root equation"),
                };

                (value, unknown_monkey)
            }
            (None, Some(y)) => {
                let unknown_monkey = self.args.0.clone();

                let value = match self.yell_type {
                    YellType::Add => value - *y,
                    YellType::Sub => value + *y,
                    YellType::Div => value * *y,
                    YellType::Mul => value / *y,
                    YellType::Equ => panic!("shouldn't see root equation"),
                };

                (value, unknown_monkey)
            }
            _ => panic!("should only have one side undefined"),
        }
    }
}

enum Yell {
    Number(MonkeyNumber),
    Equation(YellEquation),
}

impl Yell {
    fn is_equality(&self) -> bool {
        match self {
            &Yell::Number(_) => false,
            &Yell::Equation(ref eqn) => eqn.is_equality(),
        }
    }
}

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

#[derive(PartialEq, Eq)]
enum Part {
    Part1,
    Part2,
}

const YOU_NAME: &str = "humn";

struct MonkeyInfo {
    resolved: HashMap<Monkey, MonkeyNumber>,
    unresolved: HashMap<Monkey, YellEquation>,
    deps: HashMap<Monkey, HashSet<Monkey>>,
    part: Part,
}
impl MonkeyInfo {
    fn new(part: Part) -> MonkeyInfo {
        MonkeyInfo {
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

        assert!(
            eqn.yell_type != YellType::Equ || (self.part == Part::Part2 && monkey_name == "root"),
            "shouldn't see == until part 2, on root"
        );

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

    fn find_missing_number(
        &self,
        mut known_number: MonkeyNumber,
        mut unknown_monkey: Monkey,
    ) -> MonkeyNumber {
        loop {
            if self.resolved.contains_key(&unknown_monkey) {
                return known_number;
            }

            if unknown_monkey == YOU_NAME {
                return known_number;
            }

            let unknown_eqn = self
                .unresolved
                .get(&unknown_monkey)
                .expect("unknown_monkey");

            (known_number, unknown_monkey) = unknown_eqn.solve_for(&self.resolved, known_number);
        }
    }

    fn compute_required_number(&self) -> MonkeyNumber {
        assert!(
            self.part == Part::Part2,
            "required number is only in part 2"
        );

        let root_eq = self
            .unresolved
            .get(&String::from("root"))
            .expect("root eqn");

        let (ref x, ref y) = root_eq.args;

        let (known_number, unknown_monkey) = if let Some(n) = self.resolved.get(x) {
            (*n, y.clone())
        } else {
            (*self.resolved.get(y).expect("resolved y"), x.clone())
        };

        self.find_missing_number(known_number, unknown_monkey)
    }
}

fn compute_part1_root_monkey_number(input: &str) -> MonkeyNumber {
    let mut info = MonkeyInfo::new(Part::Part1);

    for MonkeyYell { name, yell } in input.trim().lines().map(parse_monkey_yell) {
        info.process_monkey_yell(name, yell);
    }

    assert!(info.deps.is_empty());
    // graph.unresolved still has entries in it.

    *info
        .resolved
        .get(&String::from("root"))
        .expect("root monkey")
}

fn compute_part2_required_monkey_number(input: &str) -> MonkeyNumber {
    let mut info = MonkeyInfo::new(Part::Part2);

    for MonkeyYell { name, yell } in input
        .trim()
        .lines()
        .map(parse_monkey_yell)
        .filter(|&MonkeyYell { ref name, ref yell }| name != YOU_NAME || yell.is_equality())
        .map(|MonkeyYell { name, yell }| MonkeyYell {
            name: name.clone(),
            yell: if name != "root" {
                yell
            } else if let Yell::Equation(eqn) = yell {
                Yell::Equation(YellEquation {
                    yell_type: YellType::Equ,
                    args: eqn.args,
                })
            } else {
                panic!("unexpected root number");
            },
        })
    {
        info.process_monkey_yell(name, yell);
    }

    info.compute_required_number()
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

    let root_yells = compute_part1_root_monkey_number(INPUT);
    println!("Part 1 root yells: {root_yells}");
    assert_eq!(root_yells, 152);

    let required_yells = compute_part2_required_monkey_number(INPUT);
    println!("Part 2 required yells: {required_yells}");
    assert_eq!(required_yells, 301);
}

fn main() {
    static INPUT: &str = include_str!("../input");

    // Part 1.
    let root_yells = compute_part1_root_monkey_number(INPUT);
    println!("Part 1 root yells: {root_yells}");
    assert_eq!(root_yells, 75_147_370_123_646);

    // Part 2.
    let required_yells = compute_part2_required_monkey_number(INPUT);
    println!("Part 2 required yells: {required_yells}");
    assert_eq!(required_yells, 3_423_279_932_937);
}
