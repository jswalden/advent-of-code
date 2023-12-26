use itertools::Itertools;
use std::iter;

#[derive(Copy, Clone, Debug)]
struct OriginalCoord(usize);

#[derive(Copy, Clone)]
struct OriginalCoords(OriginalCoord, OriginalCoord);

impl std::fmt::Debug for OriginalCoords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("OriginalCoords")
            .field(&self.0 .0)
            .field(&self.1 .0)
            .finish()
    }
}

struct GalaxyMap {
    galaxies: Vec<OriginalCoords>,
    row_translation: Vec<u64>,
    col_translation: Vec<u64>,
}

impl GalaxyMap {
    fn new(s: &'static str, empty_expand_by: u64) -> GalaxyMap {
        let mut col_has_no_galaxies = vec![];
        let mut row_has_no_galaxies = vec![];

        let mut galaxies = vec![];

        let mut lines = s.trim().lines();

        let first = lines.next().expect("first line");
        col_has_no_galaxies.resize(first.len(), true);

        for (y, line) in iter::once(first).chain(lines).enumerate() {
            let mut no_galaxies = true;
            for (x, c) in line.chars().enumerate() {
                let galaxy = match c {
                    '.' => false,
                    '#' => {
                        galaxies.push(OriginalCoords(OriginalCoord(y), OriginalCoord(x)));
                        col_has_no_galaxies[x] = false;
                        true
                    }
                    c => panic!("bad char: {c:?}"),
                };
                no_galaxies = no_galaxies && !galaxy;
            }
            row_has_no_galaxies.push(no_galaxies);
        }

        let mut prior_expansion = 0;
        let mut col_translation: Vec<_> = col_has_no_galaxies
            .iter()
            .copied()
            .enumerate()
            .map(|(x, has_no_galaxies)| {
                let expansion = prior_expansion;
                if has_no_galaxies {
                    prior_expansion += empty_expand_by - 1;
                }
                x as u64 + expansion
            })
            .collect();

        col_translation.push(col_has_no_galaxies.len() as u64 + prior_expansion);

        let mut prior_expansion = 0;
        let mut row_translation: Vec<_> = row_has_no_galaxies
            .iter()
            .copied()
            .enumerate()
            .map(|(y, has_no_galaxies)| {
                let expansion = prior_expansion;
                if has_no_galaxies {
                    prior_expansion += empty_expand_by - 1;
                }
                y as u64 + expansion
            })
            .collect();

        row_translation.push(row_has_no_galaxies.len() as u64 + prior_expansion);

        GalaxyMap {
            galaxies,
            col_translation,
            row_translation,
        }
    }

    #[cfg(test)]
    fn col_is_empty(&self, x: OriginalCoord) -> bool {
        assert!(x.0 < self.col_translation.len() - 1);
        self.col_translation[x.0] + 1 != self.col_translation[x.0 + 1]
    }

    #[cfg(test)]
    fn row_is_empty(&self, y: OriginalCoord) -> bool {
        assert!(y.0 < self.row_translation.len() - 1);
        self.row_translation[y.0] + 1 != self.row_translation[y.0 + 1]
    }

    fn galaxy_pairs<'a>(&'a self) -> impl Iterator<Item = (OriginalCoords, OriginalCoords)> + 'a {
        Itertools::tuple_combinations(self.galaxies.iter().copied())
    }

    fn manhattan_distance_expanded(&self, c1: OriginalCoords, c2: OriginalCoords) -> u64 {
        let y_delta = self.row_translation[c1.0 .0].abs_diff(self.row_translation[c2.0 .0]) as u64;
        let x_delta = self.col_translation[c1.1 .0].abs_diff(self.col_translation[c2.1 .0]) as u64;

        x_delta + y_delta
    }

    fn sum_of_expanded_distances(&self) -> u64 {
        let mut overall_sum = 0;
        for (p1, p2) in self.galaxy_pairs() {
            let sum = self.manhattan_distance_expanded(p1, p2) as u64;
            overall_sum += sum;
        }
        overall_sum
    }
}

#[test]
fn example() {
    static INPUT: &str = "
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    // Part 1.
    let galaxy_map = GalaxyMap::new(INPUT, 2);

    assert!(galaxy_map.col_is_empty(OriginalCoord(2)));
    assert!(galaxy_map.col_is_empty(OriginalCoord(5)));
    assert!(galaxy_map.col_is_empty(OriginalCoord(8)));
    assert!(galaxy_map.row_is_empty(OriginalCoord(3)));
    assert!(galaxy_map.row_is_empty(OriginalCoord(7)));

    println!("Part 1:");
    let sum_of_dists = galaxy_map.sum_of_expanded_distances();
    println!("Sum of distances: {sum_of_dists}");
    assert_eq!(sum_of_dists, 374);

    // Part 2.
    println!("Part 2:");

    let galaxy_map = GalaxyMap::new(INPUT, 10);
    let sum_of_dists = galaxy_map.sum_of_expanded_distances();
    println!("Sum of distances if expand by 10: {sum_of_dists}");
    assert_eq!(sum_of_dists, 1030);

    let galaxy_map = GalaxyMap::new(INPUT, 100);
    let sum_of_dists = galaxy_map.sum_of_expanded_distances();
    println!("Sum of distances if expand by 10: {sum_of_dists}");
    assert_eq!(sum_of_dists, 8410);
}

fn main() {
    static INPUT: &str = include_str!("../input");

    // Part 1.
    let galaxy_map = GalaxyMap::new(INPUT, 2);

    println!("Part 1:");
    let sum_of_dists = galaxy_map.sum_of_expanded_distances();
    println!("Sum of distances: {sum_of_dists}");
    assert_eq!(sum_of_dists, 10289334);

    // Part 2.
    let galaxy_map = GalaxyMap::new(INPUT, 1_000_000);

    println!("Part 2:");
    let sum_of_dists = galaxy_map.sum_of_expanded_distances();
    println!("Sum of distances: {sum_of_dists}");
    assert_eq!(sum_of_dists, 649_862_989_626);
}
