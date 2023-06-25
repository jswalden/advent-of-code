use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;

type Coord = u8;

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Cube(Coord, Coord, Coord);

fn for_all_adjacent_cubes<F>(state: &mut FloodState, Cube(x, y, z): Cube, mut f: F)
where
    F: FnMut(&mut FloodState, Cube),
{
    if x > state.x.0 {
        f(state, Cube(x - 1, y, z));
    }
    if x < state.x.1 {
        f(state, Cube(x + 1, y, z));
    }
    if y > state.y.0 {
        f(state, Cube(x, y - 1, z));
    }
    if y < state.y.1 {
        f(state, Cube(x, y + 1, z));
    }
    if z > state.z.0 {
        f(state, Cube(x, y, z - 1));
    }
    if z < state.z.1 {
        f(state, Cube(x, y, z + 1));
    }
}

fn parse_cube_list(input: &str) -> HashSet<Cube> {
    input
        .lines()
        .map(|line| {
            let c1 = line.find(',').expect("first comma");
            let c2 = c1 + 1 + line[c1 + 1..].find(',').expect("second comma");

            Cube(
                line[0..c1].parse().expect("x"),
                line[c1 + 1..c2].parse().expect("y"),
                line[c2 + 1..].parse().expect("z"),
            )
        })
        .collect()
}

#[derive(Hash, Eq, PartialEq)]
enum Normal {
    X,
    Y,
    Z,
}

#[derive(Hash, PartialEq, Eq)]
struct Side((Coord, Coord, Coord), Normal);

fn for_all_sides<F>(&Cube(x, y, z): &Cube, mut f: F)
where
    F: FnMut(Side),
{
    f(Side((x, y, z), Normal::X));
    f(Side((x + 1, y, z), Normal::X));
    f(Side((x, y, z), Normal::Y));
    f(Side((x, y + 1, z), Normal::Y));
    f(Side((x, y, z), Normal::Z));
    f(Side((x, y, z + 1), Normal::Z));
}

fn sum_surface_area(cubes: &HashSet<Cube>) -> u64 {
    let mut sides = HashMap::<Side, ()>::new();

    for cube in cubes {
        for_all_sides(&cube, |side| match sides.entry(side) {
            Entry::Occupied(e) => {
                e.remove_entry();
            }
            Entry::Vacant(v) => {
                v.insert(());
            }
        });
    }

    sides.len() as u64
}

struct MinMax(Coord, Coord);

impl Default for MinMax {
    fn default() -> MinMax {
        MinMax(Coord::MAX, Coord::MIN)
    }
}

struct FloodState<'a> {
    x: MinMax,
    y: MinMax,
    z: MinMax,
    cubes: &'a HashSet<Cube>,
    outside_cubes: HashSet<Cube>,
}

impl<'a> FloodState<'a> {
    fn new(cubes: &HashSet<Cube>) -> FloodState {
        let mut x = MinMax::default();
        let mut y = MinMax::default();
        let mut z = MinMax::default();

        for cube in cubes {
            x.0 = x.0.min(cube.0);
            x.1 = x.1.max(cube.0);
            y.0 = y.0.min(cube.1);
            y.1 = y.1.max(cube.1);
            z.0 = z.0.min(cube.2);
            z.1 = z.1.max(cube.2);
        }

        FloodState {
            x,
            y,
            z,
            cubes,
            outside_cubes: HashSet::new(),
        }
    }

    fn enclosed_surface_area(mut self) -> u64 {
        fn flood(state: &mut FloodState, outside_cube: Cube) {
            if state.cubes.contains(&outside_cube) || state.outside_cubes.contains(&&outside_cube) {
                return;
            }

            state.outside_cubes.insert(outside_cube);
            for_all_adjacent_cubes(state, outside_cube, flood);
        }

        // x=min, x=max planes
        for z in self.z.0..=self.z.1 {
            for y in self.y.0..=self.y.1 {
                let MinMax(min_x, max_x) = self.x;
                flood(&mut self, Cube(min_x, y, z));
                flood(&mut self, Cube(max_x, y, z));
            }
        }

        // y=min, y=max planes
        for z in self.z.0..=self.z.1 {
            for x in self.x.0..=self.x.1 {
                let MinMax(min_y, max_y) = self.y;
                flood(&mut self, Cube(x, min_y, z));
                flood(&mut self, Cube(x, max_y, z));
            }
        }

        // z=min, z=max planes
        for x in self.x.0..=self.x.1 {
            for y in self.y.0..=self.y.1 {
                let MinMax(min_z, max_z) = self.z;
                flood(&mut self, Cube(x, y, min_z));
                flood(&mut self, Cube(x, y, max_z));
            }
        }

        let mut enclosed = HashSet::new();

        for x in self.x.0..=self.x.1 {
            for y in self.y.0..=self.y.1 {
                for z in self.z.0..=self.z.1 {
                    let cube = Cube(x, y, z);
                    if !self.outside_cubes.contains(&cube) {
                        enclosed.insert(cube);
                    }
                }
            }
        }

        sum_surface_area(&enclosed)
    }
}

fn sum_exterior_surface_area(cubes: &HashSet<Cube>) -> u64 {
    let state = FloodState::new(cubes);
    state.enclosed_surface_area()
}

fn part1(cubes: &HashSet<Cube>, expected_surface_area: u64) {
    let surface_area = sum_surface_area(cubes);
    println!("Part 1 surface area: {surface_area}");
    assert_eq!(surface_area, expected_surface_area);
}

fn part2(cubes: &HashSet<Cube>, expected_surface_area: u64) {
    let surface_area = sum_exterior_surface_area(cubes);
    println!("Part 2 surface area: {surface_area}");
    assert_eq!(surface_area, expected_surface_area);
}

#[test]
fn basic_sides() {
    let mut one_cube = HashSet::new();
    one_cube.insert(Cube(1, 1, 1));
    assert_eq!(sum_surface_area(&one_cube), 6);
    assert_eq!(sum_exterior_surface_area(&one_cube), 6);

    let mut two_adjacent_cubes = HashSet::new();
    two_adjacent_cubes.extend(vec![Cube(1, 1, 1), Cube(2, 1, 1)]);
    assert_eq!(sum_surface_area(&two_adjacent_cubes), 10);
    assert_eq!(sum_exterior_surface_area(&two_adjacent_cubes), 10);
}

#[test]
fn example() {
    static INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    let cubes = parse_cube_list(INPUT);

    part1(&cubes, 64);
    part2(&cubes, 58);
}

fn main() {
    static INPUT: &str = include_str!("../input");

    let cubes = parse_cube_list(INPUT);

    part1(&cubes, 4364);
    part2(&cubes, 2508);
}
