enum Instruction {
    Noop,
    Addx(i32),
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    s.lines()
        .map(|line| {
            let mut it = line.split(' ');

            let first = it.next().expect("instruction name");
            match first {
                "noop" => Instruction::Noop,
                "addx" => {
                    let second = it.next().expect("addend");
                    let sign = if second.starts_with('-') { -1 } else { 1 };
                    let val = &second[if sign < 0 { 1 } else { 0 }..];
                    Instruction::Addx(sign * val.parse::<i32>().expect("number"))
                }
                s => panic!("unexpected instruction: {}", s),
            }
        })
        .collect()
}

fn run_instruction(
    cycle: &mut u32,
    signal: &mut i32,
    strength_sum: &mut i64,
    inst: &Instruction,
    is_pertinent_cycle: &dyn Fn(u32) -> bool,
    cycle_action: &mut dyn FnMut(u32, i32),
) {
    macro_rules! one_cycle {
        () => {
            cycle_action(*cycle, *signal);
            if is_pertinent_cycle(*cycle) {
                *strength_sum += *cycle as i64 * *signal as i64;
            }
            *cycle += 1;
        };
    }

    match inst {
        Instruction::Noop => {
            one_cycle!();
        }
        Instruction::Addx(amount) => {
            one_cycle!();
            one_cycle!();
            *signal += amount;
        }
    }
}

fn sum_strengths_every_twenty(insts: &Vec<Instruction>) -> i64 {
    let mut signal = 1;
    let mut cycle = 1;

    fn is_pertinent_cycle(cycle: u32) -> bool {
        cycle == 20 || (20 < cycle && cycle < 221 && (cycle - 20) % 40 == 0)
    }

    let mut strength_sum = 0;
    for inst in insts {
        run_instruction(
            &mut cycle,
            &mut signal,
            &mut strength_sum,
            inst,
            &is_pertinent_cycle,
            &mut |_cycle, _signal| {},
        );
    }

    println!("{}", strength_sum);
    strength_sum
}

const SCREEN_WIDTH: u32 = 40;
const SCREEN_HEIGHT: u32 = 6;

type Screen = Vec<Vec<char>>;

fn draw_screen(insts: &Vec<Instruction>) -> Screen {
    let mut screen = vec![vec!['.'; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];

    let mut signal = 1;
    let mut cycle = 1;
    let mut strength_sum = 0;
    for inst in insts {
        run_instruction(
            &mut cycle,
            &mut signal,
            &mut strength_sum,
            inst,
            &|_cycle| false,
            &mut |cycle, signal| {
                let row = (cycle - 1) / SCREEN_WIDTH;
                let col = (cycle - 1) % SCREEN_WIDTH;

                if (signal as i64 - col as i64).abs() < 2 {
                    screen[row as usize][col as usize] = '#';
                }
            },
        );
    }

    screen
}

#[test]
fn test_example_twenty_cycles() {}

fn display_screen(screen: &Screen) {
    for line in screen {
        for pixel in line {
            print!("{}", pixel);
        }
        println!("");
    }
}

fn main() {
    let insts = parse_instructions(include_str!("../input"));

    let signal_strength_sum = sum_strengths_every_twenty(&insts);
    assert!(signal_strength_sum == 14760);
    println!(
        "Sum of strengths at cycle 20, then 40 thereafter to 220: {}",
        signal_strength_sum
    );

    let example_insts = parse_instructions(
        "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop",
    );
    let mut screen = draw_screen(&example_insts);
    display_screen(&screen);

    println!("Part 2 screen:");
    let mut screen = draw_screen(&insts);
    display_screen(&screen);
}
