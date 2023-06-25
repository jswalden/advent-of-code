use std::collections::VecDeque;
use std::fmt;
use std::time::Instant;

fn time_call<F>(msg: &str, f: F)
where
    F: Fn(),
{
    println!("Starting {msg}...");
    let before = Instant::now();
    f();
    let elapsed = before.elapsed();
    println!("Done {msg}; elapsed: {elapsed:?}");
}

type ResourceCount = u16;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
struct Resources {
    ore: ResourceCount,
    clay: ResourceCount,
    obsidian: ResourceCount,
    geode: ResourceCount,
}

#[derive(Debug, PartialEq, Eq)]
struct Blueprint {
    id: u16,
    ore_robot_cost: Resources,
    clay_robot_cost: Resources,
    obsidian_robot_cost: Resources,
    geode_robot_cost: Resources,
}

fn parse_blueprint(s: &str) -> Blueprint {
    let id;
    let ore_robot_cost;
    let clay_robot_cost;
    let obsidian_robot_cost;
    let geode_robot_cost;

    // "Blueprint 10: "
    let bp_start = s.find("t ").expect("blueprint") + 2;
    let bp_limit = bp_start + s[bp_start..].find(':').expect("bp colon");
    id = s[bp_start..bp_limit].parse().expect("bp id");

    // "Each ore robot costs 4 ore. "
    let oc_start = s.find("costs ").expect("oc_start") + 6;
    let oc_limit = oc_start + s[oc_start..].find(' ').expect("ore space");
    ore_robot_cost = Resources {
        ore: s[oc_start..oc_limit].parse().expect("ore cost"),
        clay: 0,
        obsidian: 0,
        geode: 0,
    };

    // "Each clay robot costs 4 ore. "
    let cc_start = oc_limit + s[oc_limit..].find("costs ").expect("cc_start") + 6;
    let cc_limit = cc_start + s[cc_start..].find(' ').expect("clay space");
    clay_robot_cost = Resources {
        ore: s[cc_start..cc_limit].parse().expect("clay cost"),
        clay: 0,
        obsidian: 0,
        geode: 0,
    };

    // "Each obsidian robot costs 4 ore and 20 clay. "
    let oco_start = cc_limit + s[cc_limit..].find("costs ").expect("oco_start") + 6;
    let oco_limit = oco_start + s[oco_start..].find(' ').expect("obsidian ore space");

    let occ_start = oco_limit + s[oco_limit..].find(" and ").expect("occ_start") + 5;
    let occ_limit = occ_start + s[occ_start..].find(' ').expect("obsidian space");

    obsidian_robot_cost = Resources {
        ore: s[oco_start..oco_limit].parse().expect("obsidian ore cost"),
        clay: s[occ_start..occ_limit].parse().expect("obsidian clay cost"),
        obsidian: 0,
        geode: 0,
    };

    // "Each geode robot costs 2 ore and 12 obsidian."
    let gco_start = occ_limit + s[occ_limit..].find("costs ").expect("gco_start") + 6;
    let gco_limit = gco_start + s[gco_start..].find(' ').expect("gco_limit");

    let gcob_start = gco_limit + s[gco_limit..].find(" and ").expect("gcob_start") + 5;
    let gcob_limit = gcob_start + s[gcob_start..].find(' ').expect("gcob_limit");

    geode_robot_cost = Resources {
        ore: s[gco_start..gco_limit].parse().expect("geode ore cost"),
        clay: 0,
        obsidian: s[gcob_start..gcob_limit]
            .parse()
            .expect("geode obsidian cost"),
        geode: 0,
    };

    Blueprint {
        id,
        ore_robot_cost,
        clay_robot_cost,
        obsidian_robot_cost,
        geode_robot_cost,
    }
}

fn parse_input(input: &str, splitter: &str) -> Vec<Blueprint> {
    input
        .trim()
        .split(splitter)
        .into_iter()
        .map(parse_blueprint)
        .collect()
}

impl Resources {
    fn withdraw(&mut self, amount: &Resources) -> bool {
        if self.ore >= amount.ore
            && self.clay >= amount.clay
            && self.obsidian >= amount.obsidian
            && self.geode >= amount.geode
        {
            self.ore -= amount.ore;
            self.clay -= amount.clay;
            self.obsidian -= amount.obsidian;
            self.geode -= amount.geode;
            true
        } else {
            false
        }
    }
}

type RobotCount = u16;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Robots {
    ore: RobotCount,
    clay: RobotCount,
    obsidian: RobotCount,
    geode: RobotCount,
}

const COMPUTE_PATH: bool = false;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct State {
    time: u8,
    resources: Resources,
    robots: Robots,
    path: Vec<String>,
}

impl State {
    fn minutes(&mut self, count: u8) {
        self.time += count;

        self.resources.ore += count as u16 * self.robots.ore;
        self.resources.clay += count as u16 * self.robots.clay;
        self.resources.obsidian += count as u16 * self.robots.obsidian;
        self.resources.geode += count as u16 * self.robots.geode;
        if COMPUTE_PATH {
            self.path.push(format!("{self}"));
        }
    }

    fn one_minute(&mut self) {
        self.time += 1;

        self.resources.ore += self.robots.ore;
        self.resources.clay += self.robots.clay;
        self.resources.obsidian += self.robots.obsidian;
        self.resources.geode += self.robots.geode;
        if COMPUTE_PATH {
            self.path.push(format!("{self}"));
        }
    }

    fn advance_to_end(&self, time_limit: u8) -> ResourceCount {
        let mut state = self.clone();
        state.minutes(time_limit - state.time);
        state.resources.geode
    }

    fn buy_ore_robot(&self, blueprint: &Blueprint, time_limit: u8) -> Option<State> {
        let mut state = self.clone();
        while state.time < time_limit {
            if state.resources.withdraw(&blueprint.ore_robot_cost) {
                state.one_minute();
                state.robots.ore += 1;
                return Some(state);
            }

            state.one_minute();
        }

        None
    }

    fn buy_clay_robot(&self, blueprint: &Blueprint, time_limit: u8) -> Option<State> {
        let mut state = self.clone();
        while state.time < time_limit {
            if state.resources.withdraw(&blueprint.clay_robot_cost) {
                state.one_minute();
                state.robots.clay += 1;
                return Some(state);
            }

            state.one_minute();
        }

        None
    }

    fn buy_obsidian_robot(&self, blueprint: &Blueprint, time_limit: u8) -> Option<State> {
        let mut state = self.clone();
        while state.time < time_limit {
            if state.resources.withdraw(&blueprint.obsidian_robot_cost) {
                state.one_minute();
                state.robots.obsidian += 1;
                return Some(state);
            }

            state.one_minute();
        }

        None
    }

    fn buy_geode_robot(&self, blueprint: &Blueprint, time_limit: u8) -> Option<State> {
        let mut state = self.clone();
        while state.time < time_limit {
            if state.resources.withdraw(&blueprint.geode_robot_cost) {
                state.one_minute();
                state.robots.geode += 1;
                return Some(state);
            }

            state.one_minute();
        }

        None
    }

    fn max_geodes_achievable(&self, time_limit: u8) -> ResourceCount {
        let mut geodes = self.resources.geode;
        for i in 0..((time_limit - self.time) as u16) {
            geodes += i + self.robots.geode;
        }
        geodes
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let t = self.time;

        let Resources {
            ore,
            clay,
            obsidian,
            geode: geodes,
        } = self.resources;
        let mut resources = vec![];
        if ore > 0 {
            resources.push(format!("ore={ore}"));
        }
        if clay > 0 {
            resources.push(format!("clay={clay}"));
        }
        if obsidian > 0 {
            resources.push(format!("obsidian={obsidian}"));
        }
        if geodes > 0 {
            resources.push(format!("geodes={geodes}"));
        }

        let Robots {
            ore,
            clay,
            obsidian,
            geode,
        } = self.robots;
        let mut robots = vec![];
        if ore > 0 {
            robots.push(format!("ore={ore}"));
        }
        if clay > 0 {
            robots.push(format!("clay={clay}"));
        }
        if obsidian > 0 {
            robots.push(format!("obsidian={obsidian}"));
        }
        if geode > 0 {
            robots.push(format!("geode={geode}"));
        }

        let robots = robots.join(", ");
        let resources = resources.join(", ");
        write!(f, "t={t}, resources: {{{resources}}}, robots: {{{robots}}}")
    }
}

const DEBUG: bool = false;

fn compute_max_geodes(blueprint: &Blueprint, time_limit: u8) -> u16 {
    let mut states = VecDeque::with_capacity(1_048_576);
    states.push_back(State {
        time: 0,
        resources: Resources {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
        robots: Robots {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
        path: if COMPUTE_PATH {
            vec!["t=0, resources: {}, robots: {ore=1}".to_string()]
        } else {
            vec![]
        },
    });

    let mut max_geodes = 0;
    let mut best_path = vec![];

    let mut i = 0;
    while let Some(state) = states.pop_front() {
        if DEBUG {
            if i % 1_000_000 == 0 {
                println!("After {i} states: max_geodes={max_geodes}");
            }
            i += 1;

            println!("Simulating {state}");
        }

        // If no more robot purchases, process the number of geodes mined.
        {
            let geodes_with_no_further_buys = state.advance_to_end(time_limit);
            if geodes_with_no_further_buys > max_geodes {
                max_geodes = max_geodes.max(geodes_with_no_further_buys);
                if COMPUTE_PATH {
                    best_path = state.path.clone();
                }
                if DEBUG {
                    println!("new max_geodes={max_geodes}");
                    if COMPUTE_PATH {
                        println!("best path:");
                        for act in &best_path {
                            println!("{act}");
                        }
                    }
                }
            }
        }

        // Prune away any state that can never exceed current max geodes.
        {
            let upper_bound = state.max_geodes_achievable(time_limit);
            if upper_bound <= max_geodes {
                continue;
            }
        }

        // Buy ore robot.
        if let Some(new_sim) = state.buy_ore_robot(blueprint, time_limit) {
            if DEBUG {
                println!("  adding {new_sim}");
            }
            states.push_back(new_sim);
        }

        // Buy clay robot.
        if let Some(new_sim) = state.buy_clay_robot(blueprint, time_limit) {
            if DEBUG {
                println!("  adding {new_sim}");
            }
            states.push_back(new_sim);
        }

        // Buy obsidian robot.
        if let Some(new_sim) = state.buy_obsidian_robot(blueprint, time_limit) {
            if DEBUG {
                println!("  adding {new_sim}");
            }
            states.push_back(new_sim);
        }

        // Buy geode robot.
        if let Some(new_sim) = state.buy_geode_robot(blueprint, time_limit) {
            if DEBUG {
                println!("  adding {new_sim}");
            }
            states.push_back(new_sim);
        }
    }

    println!("max geodes: {max_geodes}");
    if COMPUTE_PATH {
        println!("path:");
        for act in best_path {
            println!("{act}");
        }
    }
    max_geodes
}

const PART1_TIME: u8 = 24;

fn quality_level(blueprint: &Blueprint, time_limit: u8) -> u16 {
    let id = blueprint.id;
    let max_geodes = compute_max_geodes(blueprint, time_limit);
    id * max_geodes
}

fn sum_quality_levels(blueprints: &Vec<Blueprint>, time: u8) -> u16 {
    blueprints.iter().map(|bp| quality_level(bp, time)).sum()
}

const PART2_TIME: u8 = 32;

fn first_three_max_geodes_product(blueprints: &Vec<Blueprint>) -> u16 {
    blueprints
        .iter()
        .take(3)
        .map(|bp| compute_max_geodes(bp, PART2_TIME))
        .product()
}

#[test]
fn test_example() {
    static INPUT: &str = "Blueprint 1:
    Each ore robot costs 4 ore.
    Each clay robot costs 2 ore.
    Each obsidian robot costs 3 ore and 14 clay.
    Each geode robot costs 2 ore and 7 obsidian.

  Blueprint 2:
    Each ore robot costs 2 ore.
    Each clay robot costs 3 ore.
    Each obsidian robot costs 3 ore and 8 clay.
    Each geode robot costs 3 ore and 12 obsidian.";

    let blueprints = parse_input(INPUT, "\n\n");
    assert_eq!(
        blueprints,
        vec![
            Blueprint {
                id: 1,
                ore_robot_cost: Resources {
                    ore: 4,
                    clay: 0,
                    obsidian: 0,
                    geode: 0
                },
                clay_robot_cost: Resources {
                    ore: 2,
                    clay: 0,
                    obsidian: 0,
                    geode: 0
                },
                obsidian_robot_cost: Resources {
                    ore: 3,
                    clay: 14,
                    obsidian: 0,
                    geode: 0
                },
                geode_robot_cost: Resources {
                    ore: 2,
                    obsidian: 7,
                    clay: 0,
                    geode: 0,
                },
            },
            Blueprint {
                id: 2,
                ore_robot_cost: Resources {
                    ore: 2,
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                },
                clay_robot_cost: Resources {
                    ore: 3,
                    clay: 0,
                    obsidian: 0,
                    geode: 0
                },
                obsidian_robot_cost: Resources {
                    ore: 3,
                    clay: 8,
                    obsidian: 0,
                    geode: 0
                },
                geode_robot_cost: Resources {
                    ore: 3,
                    obsidian: 12,
                    clay: 0,
                    geode: 0
                },
            }
        ]
    );

    if false {
        // Part 1.
        time_call("Blueprint 1 part 1 quality level", || {
            let bp1_ql = quality_level(&blueprints[0], PART1_TIME);
            println!("Blueprint 1 quality level (in t={PART1_TIME}): {bp1_ql}");
            assert_eq!(bp1_ql, 9);
        });

        time_call("Blueprint 2 part 1 quality level", || {
            let bp2_ql = quality_level(&blueprints[1], PART1_TIME);
            println!("Blueprint 2 quality level (in t={PART1_TIME}): {bp2_ql}");
            assert_eq!(bp2_ql, 24);
        });

        // Part 2.
        time_call("Blueprint 1 part 2 max geodes", || {
            let bp1_geodes = compute_max_geodes(&blueprints[0], PART2_TIME);
            println!("Blueprint 1 max geodes (in t={PART2_TIME}): {bp1_geodes}");
            assert_eq!(bp1_geodes, 56);
        });
    }

    time_call("Blueprint 2 part 2 max geodes", || {
        let bp2_geodes = compute_max_geodes(&blueprints[1], PART2_TIME);
        println!("Blueprint 2 max geodes (in t={PART2_TIME}): {bp2_geodes}");
        assert_eq!(bp2_geodes, 62);
    });
}

fn main() {
    static INPUT: &str = include_str!("../input");

    let blueprints = parse_input(INPUT, "\n");

    // Part 1.
    time_call("Part 1", || {
        let qsum = sum_quality_levels(&blueprints, PART1_TIME);
        println!("All blueprints quality levels sum: {qsum}");
        assert_eq!(qsum, 2160);
    });

    // Part 2.
    time_call("Part 2", || {
        let first_three_product = first_three_max_geodes_product(&blueprints);
        println!("First three blueprints max geodes product: {first_three_product}");
        assert_eq!(first_three_product, 13_340);
    });
}
