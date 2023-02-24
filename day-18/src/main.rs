use std::collections::HashMap;

type Coord = u8;

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug)]
struct Cube(Coord, Coord, Coord);

fn parse_cube_list(input: &str) -> Vec<Cube> {
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

fn sum_surface_area(cubes: &Vec<Cube>) -> u64 {
    let mut sides = HashMap::<Side, u8>::new();

    for cube in cubes {
        for_all_sides(&cube, |side| {
            sides
                .entry(side)
                .and_modify(|count| {
                    assert!(*count == 1, "can only see side one prior time");
                    *count += 1;
                })
                .or_insert(1);
        });
    }

    sides.iter().filter(|(_side, count)| **count == 1).count() as u64
}

fn sum_exterior_surface_area(cubes: &Vec<Cube>) -> u64 {
    sum_surface_area(cubes)
}

fn part1(cubes: &Vec<Cube>, expected_surface_area: u64) {
    let surface_area = sum_surface_area(cubes);
    println!("Part 1 surface area: {surface_area}");
    assert_eq!(surface_area, expected_surface_area);
}

fn part2(cubes: &Vec<Cube>, expected_surface_area: u64) {
    let surface_area = sum_exterior_surface_area(cubes);
    println!("Part 2 surface area: {surface_area} (expected {expected_surface_area})");
    //assert_eq!(surface_area, expected_surface_area);
}

#[test]
fn basic_sides() {
    let one_cube = vec![Cube(1, 1, 1)];
    assert_eq!(sum_surface_area(&one_cube), 6);

    let two_adjacent_cubes = vec![Cube(1, 1, 1), Cube(2, 1, 1)];
    assert_eq!(sum_surface_area(&two_adjacent_cubes), 10);
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
    part2(&cubes, 999999999);
}
