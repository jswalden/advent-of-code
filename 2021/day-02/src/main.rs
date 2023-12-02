fn parse_direction(direction: &str) -> (i32, i32) {
    let mut parts = direction.split(' ');

    let dir = parts.next().unwrap();
    let amount = parts.next().unwrap().parse().unwrap();

    match dir {
        "forward" => (amount, 0),
        "down" => (0, amount),
        "up" => (0, -amount),
        d => panic!("unexpected direction: {d}"),
    }
}

enum Command {
    Down(i32),
    Up(i32),
    Forward(i32),
}

fn parse_direction_with_aim(direction: &str) -> Command {
    let mut parts = direction.split(' ');

    let dir = parts.next().unwrap();
    let amount = parts.next().unwrap().parse().unwrap();

    match dir {
        "down" => Command::Down(amount),
        "up" => Command::Up(amount),
        "forward" => Command::Forward(amount),
        d => panic!("unexpected direction: {d}"),
    }
}

fn final_position(directions: &str) -> (i32, i32) {
    directions
        .lines()
        .map(parse_direction)
        .fold((0, 0), |(pos_x, pos_y), (delta_x, delta_y)| {
            (pos_x + delta_x, pos_y + delta_y)
        })
}

fn final_position_with_aim(directions: &str) -> (i32, i32, i32) {
    directions
        .lines()
        .map(parse_direction_with_aim)
        .fold((0, 0, 0), |(pos_x, pos_y, aim), cmd| match cmd {
            Command::Down(amount) => (pos_x, pos_y, aim + amount),
            Command::Up(amount) => (pos_x, pos_y, aim - amount),
            Command::Forward(amount) => (pos_x + amount, pos_y + aim * amount, aim),
        })
}

#[test]
fn example() {
    static INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    // Part 1.
    let (final_pos_x, final_pos_y) = final_position(INPUT);
    let mul = final_pos_x * final_pos_y;
    println!("Final position: {final_pos_x}, {final_pos_y}");
    println!("Multiply: {mul}");
    assert_eq!(mul, 150);

    // Part 2.
    let (final_pos_x, final_pos_y, final_aim) = final_position_with_aim(INPUT);
    let mul = final_pos_x * final_pos_y;
    println!("Final position: {final_pos_x}, {final_pos_y}");
    println!("Final aim: {final_aim}");
    println!("Multiply: {mul}");
    assert_eq!(mul, 900);
}

fn main() {
    static INPUT: &str = include_str!("../input");

    // Part 1.
    let (final_pos_x, final_pos_y) = final_position(INPUT);
    let mul = final_pos_x * final_pos_y;
    println!("Final position: {final_pos_x}, {final_pos_y}");
    println!("Multiply: {mul}");
    assert_eq!(mul, 1_698_735);

    // Part 2.
    let (final_pos_x, final_pos_y, final_aim) = final_position_with_aim(INPUT);
    let mul = final_pos_x * final_pos_y;
    println!("Final position: {final_pos_x}, {final_pos_y}");
    println!("Final aim: {final_aim}");
    println!("Multiply: {mul}");
    assert_eq!(mul, 1_594_785_890);
}
