mod matrix2d;

use matrix2d::Matrix2D;
use std::collections::HashMap;
use std::ops::Add;

#[derive(Copy, Clone, PartialEq, Eq)]
struct FlowRate(u32);

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Time(u32);

#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq)]
struct Dist(u32);

impl Dist {
    fn is_infinite(&self) -> bool {
        self.0 == u32::MAX
    }
}

impl Default for Dist {
    fn default() -> Self {
        Dist(u32::MAX)
    }
}

impl Add for Dist {
    type Output = Dist;

    fn add(self, rhs: Self) -> Self::Output {
        Dist(self.0.saturating_add(rhs.0))
    }
}

struct Valve {
    name: &'static str,
    flow_rate: FlowRate,
    connects_to: Vec<&'static str>,
}

const VALVE_PREFIX: &str = "Valve ";
const VALVE_PREFIX_LEN: usize = VALVE_PREFIX.len();

const HAS_FLOW_RATE_EQ: &str = " has flow rate=";
const HAS_FLOW_RATE_EQ_LEN: usize = HAS_FLOW_RATE_EQ.len();

const TO_VALVE: &str = "to valve";
const TO_VALVE_LEN: usize = TO_VALVE.len();

fn parse_valve_list(s: &'static str) -> Vec<Valve> {
    let mut valves = vec![];

    for line in s.lines() {
        // Valve WL has flow rate=7; tunnels lead to valves OQ, VN, PU, VF, UA
        // Valve YJ has flow rate=16; tunnel leads to valve YX
        let valve_name_start = VALVE_PREFIX_LEN;
        let valve_name_limit = line.find(&HAS_FLOW_RATE_EQ).expect("has flow rate=");

        let flow_rate_start = valve_name_limit + HAS_FLOW_RATE_EQ_LEN;
        let flow_rate_limit = flow_rate_start
            + line[flow_rate_start..]
                .find(|c: char| !c.is_digit(10))
                .expect("nondigit after digits");

        let valves_start = {
            let to_valve_start = flow_rate_limit
                + line[flow_rate_limit..]
                    .find(&TO_VALVE)
                    .expect("to valve(s)");

            let valve_list_start = to_valve_start + TO_VALVE_LEN;
            valve_list_start
                + line[valve_list_start..]
                    .find(' ')
                    .expect("space before valve list")
                + 1
        };

        let valve = Valve {
            name: &line[valve_name_start..valve_name_limit],
            flow_rate: FlowRate(
                line[flow_rate_start..flow_rate_limit]
                    .parse::<u32>()
                    .expect("flow rate"),
            ),
            connects_to: line[valves_start..].split(", ").collect(),
        };

        valves.push(valve);
    }
    valves
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct ValveId(usize);

struct ValveInfo {
    name: &'static str,
    flow_rate: FlowRate,
    connects_to: Vec<ValveId>,
}

struct ValveGraph {
    name_to_valve: HashMap<&'static str, ValveId>,
    valves: Vec<ValveInfo>,
    starting_valve: ValveId,
}

struct ValveEdges<'a> {
    valves: &'a Vec<ValveInfo>,
    curr_valve: usize,
    curr_connection: usize,
}

impl<'a> Iterator for ValveEdges<'a> {
    type Item = (ValveId, ValveId);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.curr_valve >= self.valves.len() {
                return None;
            }

            let v = &self.valves[self.curr_valve];
            if self.curr_connection >= v.connects_to.len() {
                self.curr_valve += 1;
                self.curr_connection = 0;
                continue;
            }

            let curr_valve = self.curr_valve;
            let curr_connection = self.curr_connection;
            self.curr_connection += 1;
            if self.curr_connection == v.connects_to.len() {
                self.curr_valve += 1;
                self.curr_connection = 0;
            }

            return Some((ValveId(curr_valve), v.connects_to[curr_connection]));
        }
    }
}

impl ValveGraph {
    #[allow(dead_code)]
    fn valve_to_name(&self, valve: ValveId) -> &'static str {
        let ValveId(index) = valve;
        self.valves[index].name
    }

    fn name_to_valve(&self, name: &'static str) -> Option<ValveId> {
        self.name_to_valve.get(name).copied()
    }

    fn valve_count(&self) -> usize {
        self.valves.len()
    }

    fn edges(&self) -> ValveEdges {
        ValveEdges {
            valves: &self.valves,
            curr_valve: 0,
            curr_connection: 0,
        }
    }

    #[allow(dead_code)]
    fn valves(&self) -> impl Iterator<Item = ValveId> {
        (0..self.valves.len()).map(ValveId)
    }

    fn valves_from<'a>(&'a self, from: ValveId) -> impl Iterator<Item = ValveId> + 'a {
        let ValveId(from) = from;
        self.valves[from].connects_to.iter().copied()
    }
}

fn build_valve_graph(s: &'static str) -> ValveGraph {
    let valve_list = parse_valve_list(s);

    let mut name_to_valve = HashMap::new();
    let mut valves = vec![];

    let mut starting_valve = None;

    for Valve {
        name, flow_rate, ..
    } in &valve_list
    {
        let index = ValveId(valves.len());
        if *name == "AA" {
            starting_valve = Some(index);
        }

        let prev = name_to_valve.insert(*name, index);
        assert!(prev.is_none(), "shouldn't have repeated node");

        valves.push(ValveInfo {
            name: *name,
            flow_rate: *flow_rate,
            connects_to: vec![],
        });
    }

    let starting_valve = starting_valve.expect("expect an AA node");

    for (i, desc) in valve_list.iter().enumerate() {
        let connects_to = &mut valves[i].connects_to;
        connects_to.extend(
            desc.connects_to
                .iter()
                .map(|name| name_to_valve.get(name).expect("present")),
        );
    }

    ValveGraph {
        name_to_valve,
        valves,
        starting_valve,
    }
}

#[derive(Copy, Clone)]
struct PathElem {
    dist: Dist,
    next: usize,
}

impl Default for PathElem {
    fn default() -> Self {
        PathElem {
            dist: Default::default(),
            next: usize::MAX,
        }
    }
}

struct ShortestDistanceInfo {
    matrix: Matrix2D<PathElem>,
}

impl ShortestDistanceInfo {
    #[allow(dead_code)]
    fn shortest_path(
        &self,
        graph: &ValveGraph,
        from_name: &'static str,
        to_name: &'static str,
    ) -> Option<Vec<&'static str>> {
        let ValveId(from) = graph.name_to_valve(from_name)?;
        let ValveId(to) = graph.name_to_valve(to_name)?;

        let mut path_elem = self.matrix[(from, to)];
        if path_elem.dist == Default::default() {
            return None;
        }

        let mut edges = vec![from_name];
        loop {
            let dist = path_elem.dist;
            if dist == Dist(0) {
                break;
            }

            let next = path_elem.next;
            edges.push(graph.valve_to_name(ValveId(next)));
            path_elem = self.matrix[(next, to)];
        }

        Some(edges)
    }

    fn distance(&self, from: ValveId, to: ValveId) -> Dist {
        assert!(from.0 < self.matrix.width);
        assert!(to.0 < self.matrix.width);

        let path_elem = &self.matrix[(from.0, to.0)];
        assert!(
            path_elem.next != usize::MAX,
            "can't request distance between unconnected valves"
        );

        path_elem.dist
    }

    fn new(graph: &ValveGraph) -> ShortestDistanceInfo {
        let num_valves = graph.valve_count();

        let mut matrix = Matrix2D::<PathElem>::new(num_valves, num_valves);

        // For every tunnel, note distance 1 between valves with destination valve
        // as next valve to visit.
        for (ValveId(from), ValveId(to)) in graph.edges() {
            matrix[(from, to)] = PathElem {
                dist: Dist(1),
                next: to,
            };
        }

        // No distance or next valve for each valve to itself.
        for i in 0..num_valves {
            matrix[(i, i)] = PathElem {
                dist: Dist(0),
                next: usize::MAX,
            };
        }

        // Extend paths out from each valve.
        for k in 0..num_valves {
            for i in 0..num_valves {
                for j in 0..num_valves {
                    let ik_dist = matrix[(i, k)].dist;
                    let kj_dist = matrix[(k, j)].dist;
                    let cand_dist = ik_dist + kj_dist;
                    if matrix[(i, j)].dist > cand_dist {
                        matrix[(i, j)].dist = cand_dist;
                        matrix[(i, j)].next = k;
                    }
                }
            }
        }

        ShortestDistanceInfo { matrix }
    }
}

fn minimize_graph(
    graph: &ValveGraph,
    shortest_distance_info: &ShortestDistanceInfo,
) -> (ValveGraph, ShortestDistanceInfo) {
    let old_valves = &graph.valves;

    let mut new_valves = vec![];
    let mut new_name_to_new_valve = HashMap::new();

    let mut old_valve_from_new_valve = vec![];

    let mut starting_valve = None;
    for (
        old,
        &ValveInfo {
            name, flow_rate, ..
        },
    ) in old_valves.iter().enumerate()
    {
        if flow_rate.0 == 0 && name != "AA" {
            continue;
        }

        let id = ValveId(new_valves.len());
        new_name_to_new_valve.insert(name, id);

        if name == "AA" {
            starting_valve = Some(id);
        }

        new_valves.push(ValveInfo {
            name,
            flow_rate,
            connects_to: vec![],
        });
        old_valve_from_new_valve.push(ValveId(old));
    }

    let n = new_valves.len();
    let mut m = Matrix2D::new(n, n);

    let old_m = &shortest_distance_info.matrix;
    for i in 0..n {
        let old_i = old_valve_from_new_valve[i].0;
        for j in 0..n {
            let old_j = old_valve_from_new_valve[j].0;

            let old = &old_m[(old_i, old_j)];
            if !old.dist.is_infinite() {
                m[(i, j)] = PathElem {
                    dist: old.dist,
                    next: j,
                };

                new_valves[i].connects_to.push(ValveId(j));
            }
        }
    }

    let graph = ValveGraph {
        name_to_valve: new_name_to_new_valve,
        valves: new_valves,
        starting_valve: starting_valve.expect("AA"),
    };

    let sdi = ShortestDistanceInfo { matrix: m };

    (graph, sdi)
}

struct ValvesVisited(u32);

impl ValvesVisited {
    fn new(graph: &ValveGraph) -> ValvesVisited {
        assert!(graph.valve_count() < u32::BITS as usize);
        ValvesVisited(0)
    }

    fn contains(&self, valve: ValveId) -> bool {
        self.0 & Self::to_bit(valve) != 0
    }

    fn visit(&self, valve: ValveId) -> ValvesVisited {
        let bit = Self::to_bit(valve);
        ValvesVisited(self.0 | bit)
    }

    fn to_bit(valve: ValveId) -> u32 {
        1 << valve.0
    }

    fn all_max_flows(
        graph: &ValveGraph,
        shortest_distances: &ShortestDistanceInfo,
        time: Time,
    ) -> Vec<u32> {
        let mut max_flows = vec![];
        for mask in 0..(1 << (graph.valve_count())) {
            let flow = find_max_flow(graph, shortest_distances, ValvesVisited(mask), time);
            max_flows.push(flow);
        }
        max_flows
    }
}

fn sum_remaining_flows(
    graph: &ValveGraph,
    shortest_distances: &ShortestDistanceInfo,
    visited_valves: ValvesVisited,
    current_valve: ValveId,
    remaining_time: Time,
) -> u32 {
    assert!(
        visited_valves.contains(current_valve),
        "current valve must already be visited"
    );

    let mut best_additional = 0;
    for next_valve in graph.valves_from(current_valve) {
        if visited_valves.contains(next_valve) {
            continue;
        }

        let rooms_to_move = shortest_distances.distance(current_valve, next_valve);

        let time_to_travel_and_open = Time(rooms_to_move.0 + 1);

        if remaining_time <= time_to_travel_and_open {
            continue;
        }

        let next_remaining_time = Time(remaining_time.0 - time_to_travel_and_open.0);

        let next_visited_valves = visited_valves.visit(next_valve);

        let (next_current_valve, next_remaining_time) = (next_valve, next_remaining_time);

        let additional = sum_remaining_flows(
            graph,
            shortest_distances,
            next_visited_valves,
            next_current_valve,
            next_remaining_time,
        );

        best_additional = best_additional.max(additional);
    }

    let current_valve_flow = graph.valves[current_valve.0].flow_rate;
    current_valve_flow.0 * remaining_time.0 + best_additional
}

fn find_max_flow(
    graph: &ValveGraph,
    shortest_distances: &ShortestDistanceInfo,
    visited_valves: ValvesVisited,
    remaining_time: Time,
) -> u32 {
    let start = graph.starting_valve;

    let visited_valves = visited_valves.visit(start);

    sum_remaining_flows(
        graph,
        shortest_distances,
        visited_valves,
        start,
        remaining_time,
    )
}

fn find_max_flow_without_elephant(
    graph: &ValveGraph,
    shortest_distances: &ShortestDistanceInfo,
) -> u32 {
    let no_valves_visited = ValvesVisited::new(graph);
    find_max_flow(graph, shortest_distances, no_valves_visited, Time(30))
}

fn find_max_flow_with_elephant(
    graph: &ValveGraph,
    shortest_distances: &ShortestDistanceInfo,
) -> u32 {
    let max_flows = ValvesVisited::all_max_flows(graph, shortest_distances, Time(26));

    let invert_mask = |m: usize| !m & ((1 << graph.valve_count()) - 1);

    let mut max_flow = 0;
    for m1 in 0..(max_flows.len() / 2) {
        let m2 = invert_mask(m1);

        let combined_flow = max_flows[m1] + max_flows[m2];
        max_flow = max_flow.max(combined_flow);
    }

    max_flow
}

#[test]
fn test_example() {
    static CONTENT: &'static str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    let graph = build_valve_graph(CONTENT);

    let shortest_distances = ShortestDistanceInfo::new(&graph);

    assert_eq!(
        shortest_distances.shortest_path(&graph, "AA", "EE"),
        Some(["AA", "DD", "EE"].iter().copied().collect())
    );

    let (min_graph, min_shortest_distances) = minimize_graph(&graph, &shortest_distances);

    // Without elephant.
    {
        let start = std::time::Instant::now();

        let best_flow = find_max_flow_without_elephant(&min_graph, &min_shortest_distances);
        println!("(test) best flow without elephant: {}", best_flow);
        assert_eq!(best_flow, 1651);

        println!("  computed in time {:?}", start.elapsed());
    }

    // With elephant.
    {
        let start = std::time::Instant::now();

        let best_flow = find_max_flow_with_elephant(&min_graph, &min_shortest_distances);
        println!("(test) best flow with elephant: {}", best_flow);
        assert_eq!(best_flow, 1707);

        println!("  computed in time {:?}", start.elapsed());
    }
}

fn main() {
    static CONTENT: &'static str = include_str!("../input");

    let graph = build_valve_graph(CONTENT);

    let shortest_distances = ShortestDistanceInfo::new(&graph);

    let (min_graph, min_shortest_distances) = minimize_graph(&graph, &shortest_distances);

    // Without elephant.
    {
        let start = std::time::Instant::now();

        let best_flow = find_max_flow_without_elephant(&min_graph, &min_shortest_distances);
        println!("best flow without elephant: {}", best_flow);
        assert_eq!(best_flow, 1986);

        println!("  computed in time {:?}", start.elapsed());
    }

    // With elephant.
    {
        let start = std::time::Instant::now();

        let best_flow = find_max_flow_with_elephant(&min_graph, &min_shortest_distances);
        println!("best flow with elephant: {}", best_flow);
        assert_eq!(best_flow, 2464);

        println!("  computed in time {:?}", start.elapsed());
    }
}
