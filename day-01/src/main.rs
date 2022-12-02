use itertools::Itertools;

static CONTENTS: &str = include_str!("../input");

fn part1() {
    let mut maxcals = 0;
    let mut maxelf = 0;

    let mut elf = 0usize;
    let mut calories = 0;

    for line in CONTENTS.lines() {
        if line.is_empty() {
            elf += 1;

            if calories >= maxcals {
                maxcals = calories;
                maxelf = elf;
            }

            calories = 0;
            continue;
        }

        let item = line;
        let itemcals = item.parse::<usize>().unwrap();

        calories += itemcals;
    }

    elf += 1;
    if calories >= maxcals {
        maxcals = calories;
        maxelf = elf;
    }

    print!(
        "Elf {} (1-indexed) carrying total calories {}\n",
        maxelf + 1,
        maxcals
    );
}

fn part1_take2() {
    print!(
        "max cals: {}\n",
        CONTENTS
            .lines()
            .into_iter()
            .group_by(|line| (*line).is_empty())
            .into_iter()
            .filter_map(|(empty, group)| if empty { None } else { Some(group) })
            .map(|group| group.fold(0, |acc, line| acc + line.parse::<usize>().unwrap()))
            .max()
            .unwrap()
    );
}

#[derive(Copy, Clone)]
struct ElfAndCalories {
    elf: usize,
    calories: usize,
}

impl ElfAndCalories {
    fn new(elf: usize, calories: usize) -> Self {
        ElfAndCalories { elf, calories }
    }
}

#[derive(Copy, Clone)]
struct TopThree {
    lo: ElfAndCalories,
    mid: ElfAndCalories,
    hi: ElfAndCalories,
}

impl TopThree {
    fn new() -> Self {
        TopThree {
            lo: ElfAndCalories::new(0, 0),
            mid: ElfAndCalories::new(0, 0),
            hi: ElfAndCalories::new(0, 0),
        }
    }

    fn insert(&mut self, elf: usize, n: usize) {
        if n >= self.hi.calories {
            self.lo = self.mid;
            self.mid = self.hi;
            self.hi = ElfAndCalories::new(elf, n);
        } else if n >= self.mid.calories {
            self.lo = self.mid;
            self.mid = ElfAndCalories::new(elf, n);
        } else if n >= self.lo.calories {
            self.lo = ElfAndCalories::new(elf, n);
        }
    }
}

fn part2() {
    let mut top_three = TopThree::new();

    let mut elf = 0;
    let mut running_calories = 0;

    for line in CONTENTS.lines() {
        if line.is_empty() {
            elf += 1;
            top_three.insert(elf, running_calories);
            running_calories = 0;
            continue;
        }

        running_calories += line.parse::<usize>().unwrap();
    }

    elf += 1;
    top_three.insert(elf, running_calories);

    let print_elf_and_calories = |num: &str, elf_and_calories: &ElfAndCalories| {
        print!(
            "{}: elf {} ({} cal)\n",
            num, elf_and_calories.elf, elf_and_calories.calories
        );
    };

    print_elf_and_calories("1st", &top_three.hi);
    print_elf_and_calories("2nd", &top_three.mid);
    print_elf_and_calories("3rd", &top_three.lo);

    print!(
        "top three total: {}\n",
        top_three.hi.calories + top_three.mid.calories + top_three.lo.calories
    );
}

fn main() {
    part1();
    part1_take2();
    part2();
}
