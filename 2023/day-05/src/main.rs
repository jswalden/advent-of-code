#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Item(u64);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct ItemRange(Item, u64);

struct Translation {
    source: Item,
    dest: Item,
    len: u64,
}

struct Map<'a> {
    _to_cat: &'a str,
    translations: Vec<Translation>,
}

fn sort_item_ranges(translations: &mut Vec<ItemRange>) {
    translations.sort_unstable_by_key(|&ItemRange(start, _count)| start.0);
}

impl<'a> Map<'a> {
    fn translate_range(&self, mut range: ItemRange) -> Vec<ItemRange> {
        let mut translated = vec![];

        loop {
            match self.find_first_mapping(range.0) {
                Some(translation) => {
                    let (mut range_start, range_end) = (range.0 .0, range.0 .0 + range.1 - 1);
                    if range_end < translation.source.0 {
                        translated.push(range);
                        break;
                    }

                    if range_start < translation.source.0 {
                        let count = translation.source.0 - range_start;
                        if count > 0 {
                            translated.push(ItemRange(Item(range_start), count));
                            range_start = translation.source.0;
                        }
                    }

                    let start_offset = range_start - translation.source.0;
                    let translated_start = Item(translation.dest.0 + start_offset);
                    if range_end < translation.source.0 + translation.len {
                        let translated_count = range_end - range_start + 1;
                        translated.push(ItemRange(translated_start, translated_count));
                        break;
                    }

                    translated.push(ItemRange(translated_start, translation.len - start_offset));

                    range_start = translation.source.0 + translation.len;
                    range = ItemRange(Item(range_start), range_end - range_start + 1);
                }
                None => {
                    translated.push(range);
                    break;
                }
            }
        }

        sort_item_ranges(&mut translated);

        translated
    }

    fn find_first_mapping(&self, start: Item) -> Option<&Translation> {
        match self
            .translations
            .binary_search_by_key(&start.0, |&Translation { source, len, .. }| {
                source.0 + len - 1
            }) {
            Ok(idx) => Some(&self.translations[idx]),
            Err(idx) => (idx < self.translations.len()).then(|| &self.translations[idx]),
        }
    }
}

#[test]
fn translation_single() {
    let m = Map {
        _to_cat: "to_cat",
        translations: vec![Translation {
            source: Item(10),
            dest: Item(20),
            len: 3,
        }],
    };

    let translate_one = |item: Item| m.translate_range(ItemRange(item, 1))[0].0;

    assert_eq!(translate_one(Item(9)), Item(9));
    assert_eq!(translate_one(Item(10)), Item(20));
    assert_eq!(translate_one(Item(11)), Item(21));
    assert_eq!(translate_one(Item(12)), Item(22));
    assert_eq!(translate_one(Item(13)), Item(13));

    let translate_range = |range: ItemRange| m.translate_range(range);

    assert_eq!(
        translate_range(ItemRange(Item(5), 4)),
        vec![ItemRange(Item(5), 4)]
    );
    assert_eq!(
        translate_range(ItemRange(Item(5), 5)),
        vec![ItemRange(Item(5), 5)]
    );
    assert_eq!(
        translate_range(ItemRange(Item(5), 6)),
        vec![ItemRange(Item(5), 5), ItemRange(Item(20), 1)]
    );
    assert_eq!(
        translate_range(ItemRange(Item(5), 7)),
        vec![ItemRange(Item(5), 5), ItemRange(Item(20), 2)]
    );
    assert_eq!(
        translate_range(ItemRange(Item(5), 8)),
        vec![ItemRange(Item(5), 5), ItemRange(Item(20), 3)]
    );
    assert_eq!(
        translate_range(ItemRange(Item(5), 9)),
        vec![
            ItemRange(Item(5), 5),
            ItemRange(Item(13), 1),
            ItemRange(Item(20), 3)
        ]
    );
}

#[test]
fn translation_multiple() {
    let m = Map {
        _to_cat: "to_cat",
        translations: vec![
            Translation {
                source: Item(10),
                dest: Item(20),
                len: 3,
            },
            Translation {
                source: Item(15),
                dest: Item(28),
                len: 3,
            },
        ],
    };

    let translate_one = |item: Item| m.translate_range(ItemRange(item, 1))[0].0;

    assert_eq!(translate_one(Item(9)), Item(9));
    assert_eq!(translate_one(Item(10)), Item(20));
    assert_eq!(translate_one(Item(11)), Item(21));
    assert_eq!(translate_one(Item(12)), Item(22));
    assert_eq!(translate_one(Item(13)), Item(13));

    let translate_range = |range: ItemRange| m.translate_range(range);

    assert_eq!(
        translate_range(ItemRange(Item(5), 4)),
        vec![ItemRange(Item(5), 4)]
    );
    assert_eq!(
        translate_range(ItemRange(Item(5), 5)),
        vec![ItemRange(Item(5), 5)]
    );
    assert_eq!(
        translate_range(ItemRange(Item(5), 6)),
        vec![ItemRange(Item(5), 5), ItemRange(Item(20), 1)]
    );
    assert_eq!(
        translate_range(ItemRange(Item(5), 7)),
        vec![ItemRange(Item(5), 5), ItemRange(Item(20), 2)]
    );
    assert_eq!(
        translate_range(ItemRange(Item(5), 8)),
        vec![ItemRange(Item(5), 5), ItemRange(Item(20), 3)]
    );
    assert_eq!(
        translate_range(ItemRange(Item(5), 9)),
        vec![
            ItemRange(Item(5), 5),
            ItemRange(Item(13), 1),
            ItemRange(Item(20), 3)
        ]
    );
    assert_eq!(
        translate_range(ItemRange(Item(5), 10)),
        vec![
            ItemRange(Item(5), 5),
            ItemRange(Item(13), 2),
            ItemRange(Item(20), 3)
        ]
    );
    assert_eq!(
        translate_range(ItemRange(Item(5), 11)),
        vec![
            ItemRange(Item(5), 5),
            ItemRange(Item(13), 2),
            ItemRange(Item(20), 3),
            ItemRange(Item(28), 1)
        ]
    );
    assert_eq!(
        translate_range(ItemRange(Item(5), 12)),
        vec![
            ItemRange(Item(5), 5),
            ItemRange(Item(13), 2),
            ItemRange(Item(20), 3),
            ItemRange(Item(28), 2)
        ]
    );
    assert_eq!(
        translate_range(ItemRange(Item(5), 13)),
        vec![
            ItemRange(Item(5), 5),
            ItemRange(Item(13), 2),
            ItemRange(Item(20), 3),
            ItemRange(Item(28), 3)
        ]
    );
    assert_eq!(
        translate_range(ItemRange(Item(5), 14)),
        vec![
            ItemRange(Item(5), 5),
            ItemRange(Item(13), 2),
            ItemRange(Item(18), 1),
            ItemRange(Item(20), 3),
            ItemRange(Item(28), 3)
        ]
    );
}

struct Almanac<'a> {
    seeds: Vec<Item>,
    maps: Vec<Map<'a>>,
}

enum Part {
    One,
    Two,
}

fn sort_translations(translations: &mut Vec<Translation>) {
    translations.sort_unstable_by_key(|t| t.source.0);
}

impl<'a> Almanac<'a> {
    fn parse(s: &str) -> Almanac {
        let mut lines = s.lines().peekable();

        // Seeds
        let seed_line = lines.next().expect("seeds");
        let seeds: Vec<_> = seed_line
            .split_once(' ')
            .map(|(_, seeds)| seeds)
            .expect("seed_line")
            .split(' ')
            .map(|seednum| seednum.parse::<u64>().expect("seed parse"))
            .map(Item)
            .collect();

        let empty = lines.next().expect("after seeds");
        assert_eq!(empty, "");

        let mut maps = vec![];

        'all_maps: loop {
            let map_line = match lines.next() {
                Some(line) => line,
                None => break 'all_maps,
            };

            let mut parts = map_line.split('-');
            let _source = parts.next().expect("source");
            assert_eq!(parts.next().expect("to"), "to");
            let dest = parts
                .next()
                .expect("dest map:")
                .split_once(' ')
                .expect("map string")
                .0;

            let mut translations = vec![];

            'current_map: loop {
                let mut iter = lines.next().expect("translation").split(' ').peekable();
                let (dest, source, len) = (
                    Item(iter.next().expect("dest").parse::<u64>().expect("dest")),
                    Item(iter.next().expect("source").parse::<u64>().expect("source")),
                    iter.next().expect("count").parse::<u64>().expect("count"),
                );

                translations.push(Translation { source, dest, len });

                match lines.peek() {
                    None => break 'current_map,
                    Some(&"") => {
                        lines.next().expect("empty");
                        break 'current_map;
                    }
                    _ => {}
                }
            }

            sort_translations(&mut translations);

            maps.push(Map {
                _to_cat: dest,
                translations,
            })
        }

        Almanac { seeds, maps }
    }

    fn seed_ranges<'b>(&'b self, part: Part) -> impl Iterator<Item = ItemRange> + 'b {
        struct SeedRangeIter<'c> {
            almanac: &'c Almanac<'c>,
            idx: usize,
            part: Part,
        }

        impl<'c> Iterator for SeedRangeIter<'c> {
            type Item = ItemRange;

            fn next(&mut self) -> Option<Self::Item> {
                if self.idx == self.almanac.seeds.len() {
                    return None;
                }

                match self.part {
                    Part::One => {
                        let res = Some(ItemRange(self.almanac.seeds[self.idx], 1));
                        self.idx += 1;
                        res
                    }
                    Part::Two => {
                        let start = self.almanac.seeds[self.idx];
                        let len = self.almanac.seeds[self.idx + 1].0;
                        let res = Some(ItemRange(start, len));
                        self.idx += 2;
                        res
                    }
                }
            }
        }

        SeedRangeIter {
            almanac: self,
            idx: 0,
            part,
        }
    }

    fn lowest_location_seed(&self, part: Part) -> Item {
        let mut lowest = None::<Item>;

        for range in self.seed_ranges(part) {
            let mut candidates = vec![range];

            for map in &self.maps {
                let mut translated = vec![];
                for range in candidates {
                    translated.extend(map.translate_range(range));
                }

                sort_item_ranges(&mut translated);
                candidates = translated;
            }

            if let Some(x) = lowest {
                if candidates[0].0 .0 < x.0 {
                    lowest = Some(candidates[0].0);
                }
            } else {
                lowest = Some(candidates[0].0);
            }
        }

        lowest.expect("lowest_ranges")
    }
}

#[test]
fn example() {
    static INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    let almanac = Almanac::parse(INPUT);

    // Part 1.
    let lowest_location = almanac.lowest_location_seed(Part::One);
    println!("lowest_location: {lowest_location:?}");
    assert_eq!(lowest_location, Item(35));

    // Part 2.
    let lowest_location = almanac.lowest_location_seed(Part::Two);
    println!("lowest location seed ranges: {lowest_location:?}");
    assert_eq!(lowest_location, Item(46));
}

fn main() {
    static _INPUT: &str = include_str!("../input");

    let almanac = Almanac::parse(_INPUT);

    // Part 1.
    let lowest_location = almanac.lowest_location_seed(Part::One);
    println!("lowest location individual seeds: {lowest_location:?}");
    assert_eq!(lowest_location, Item(174_137_457));

    // Part 2.
    let lowest_location = almanac.lowest_location_seed(Part::Two);
    println!("lowest location seed ranges: {lowest_location:?}");
    assert_eq!(lowest_location, Item(1493866));
}
