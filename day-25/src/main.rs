type Snafu = i64;

const BASE: Snafu = 5;
const MIN: Snafu = -2;
const MAX: Snafu = 2;
const SNAFU_DIGITS: [char; BASE as usize] = ['=', '-', '0', '1', '2'];

fn number_from_snafu(snafu: &str) -> Snafu {
    snafu
        .chars()
        .map(|c| match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            c => panic!("bad SNAFU: {c:?}"),
        })
        .fold(0, |sum, d| sum * BASE + d)
}

#[test]
fn convert_to_number() {
    assert_eq!(number_from_snafu("1"), 1);
    assert_eq!(number_from_snafu("2"), 2);
    assert_eq!(number_from_snafu("1="), 3);
    assert_eq!(number_from_snafu("1-"), 4);
    assert_eq!(number_from_snafu("10"), 5);
    assert_eq!(number_from_snafu("11"), 6);
    assert_eq!(number_from_snafu("12"), 7);
    assert_eq!(number_from_snafu("2="), 8);
    assert_eq!(number_from_snafu("2-"), 9);
    assert_eq!(number_from_snafu("20"), 10);
    assert_eq!(number_from_snafu("21"), 11);
    assert_eq!(number_from_snafu("22"), 12);
    assert_eq!(number_from_snafu("1=="), 13);
    assert_eq!(number_from_snafu("1=0"), 15);
    assert_eq!(number_from_snafu("1-0"), 20);
    assert_eq!(number_from_snafu("1=11-2"), 2022);
    assert_eq!(number_from_snafu("1-0---0"), 12345);
    assert_eq!(number_from_snafu("1121-1110-1=0"), 314159265);
}

fn count_snafu_digits(n: Snafu, base: Snafu, max: Snafu) -> usize {
    let mut digit_count = 1;
    let mut upper_limit = MAX;
    while n > upper_limit {
        digit_count += 1;
        upper_limit = upper_limit * base + max;
    }

    digit_count
}

#[test]
fn count_digits() {
    assert_eq!(count_snafu_digits(0, BASE, MAX), 1);
    assert_eq!(count_snafu_digits(1, BASE, MAX), 1);
    assert_eq!(count_snafu_digits(2, BASE, MAX), 1);
    assert_eq!(count_snafu_digits(3, BASE, MAX), 2);
    assert_eq!(count_snafu_digits(4, BASE, MAX), 2);
    assert_eq!(count_snafu_digits(5, BASE, MAX), 2);
    assert_eq!(count_snafu_digits(6, BASE, MAX), 2);
    assert_eq!(count_snafu_digits(7, BASE, MAX), 2);
    assert_eq!(count_snafu_digits(8, BASE, MAX), 2);
    assert_eq!(count_snafu_digits(9, BASE, MAX), 2);
    assert_eq!(count_snafu_digits(10, BASE, MAX), 2);
    assert_eq!(count_snafu_digits(11, BASE, MAX), 2);
    assert_eq!(count_snafu_digits(12, BASE, MAX), 2);
    assert_eq!(count_snafu_digits(13, BASE, MAX), 3);
    assert_eq!(count_snafu_digits(14, BASE, MAX), 3);
    assert_eq!(count_snafu_digits(2022, BASE, MAX), 6);
    assert_eq!(count_snafu_digits(12345, BASE, MAX), 7);
    assert_eq!(count_snafu_digits(314159265, BASE, MAX), 13);
}

fn compute_envelope(digit: Snafu, base: Snafu, count: usize) -> Snafu {
    (0..count).fold(0, |sum, _| sum * base + digit)
}

#[test]
fn check_envelope() {
    assert_eq!(compute_envelope(MAX, BASE, 1), 2);
    assert_eq!(compute_envelope(MAX, BASE, 2), 12);
    assert_eq!(compute_envelope(MAX, BASE, 3), 62);
    assert_eq!(compute_envelope(MAX, BASE, 4), 312);
    assert_eq!(compute_envelope(MAX, BASE, 5), 1562);

    assert_eq!(compute_envelope(MIN, BASE, 1), -2);
    assert_eq!(compute_envelope(MIN, BASE, 2), -12);
    assert_eq!(compute_envelope(MIN, BASE, 3), -62);
    assert_eq!(compute_envelope(MIN, BASE, 4), -312);
    assert_eq!(compute_envelope(MIN, BASE, 5), -1562);
}

fn snafu_from_number(n: Snafu) -> String {
    let num_digits = count_snafu_digits(n, BASE, MAX);

    let mut snafu = String::new();

    let mut remaining = n;
    let mut env_min = compute_envelope(MIN, BASE, num_digits);
    let mut env_max = compute_envelope(MAX, BASE, num_digits);
    let mut digit_value = BASE.pow(num_digits as u32 - 1);
    for _ in 0..num_digits {
        let digit = (remaining - env_min) / digit_value;
        let snafu_digit = SNAFU_DIGITS[digit as usize];
        snafu.push(snafu_digit);

        remaining -= (digit + MIN) * digit_value;
        env_min = (env_min - MIN) / BASE;
        env_max = (env_max - MIN) / BASE;
        digit_value /= BASE;
    }

    assert_eq!(remaining, 0);
    assert_eq!(env_min, 0);
    assert_eq!(env_max, 0);
    assert_eq!(digit_value, 0);

    assert_eq!(number_from_snafu(&snafu), n, "{n} should roundtrip");

    snafu
}

#[test]
fn compute_snafus() {
    assert_eq!(snafu_from_number(0), "0");
    assert_eq!(snafu_from_number(1), "1");
    assert_eq!(snafu_from_number(2), "2");
    assert_eq!(snafu_from_number(3), "1=");
    assert_eq!(snafu_from_number(4), "1-");
    assert_eq!(snafu_from_number(5), "10");
    assert_eq!(snafu_from_number(6), "11");
    assert_eq!(snafu_from_number(7), "12");
    assert_eq!(snafu_from_number(8), "2=");
    assert_eq!(snafu_from_number(9), "2-");
    assert_eq!(snafu_from_number(10), "20");
    assert_eq!(snafu_from_number(11), "21");
    assert_eq!(snafu_from_number(12), "22");
    assert_eq!(snafu_from_number(13), "1==");
    assert_eq!(snafu_from_number(2022), "1=11-2");
    assert_eq!(snafu_from_number(12345), "1-0---0");
    assert_eq!(snafu_from_number(314159265), "1121-1110-1=0");
}

#[test]
fn example() {
    static INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    // Part 1.
    let sum = INPUT.lines().map(number_from_snafu).sum::<Snafu>();
    println!("Sum of input numbers: {sum} (should be 4890)");
    assert_eq!(sum, 4890);

    let snafu_number = snafu_from_number(sum);
    println!("SNAFU number for sum: {snafu_number}");
    assert_eq!(snafu_number, "2=-1=0");
}

fn main() {
    static INPUT: &str = include_str!("../input");

    // Part 1.
    let sum = INPUT.lines().map(number_from_snafu).sum::<Snafu>();
    println!("Sum of input numbers: {sum}");
    assert_eq!(sum, 33_078_355_623_611);

    let snafu_number = snafu_from_number(sum);
    println!("SNAFU number for sum: {snafu_number}");
    assert_eq!(snafu_number, "2-=2-0=-0-=0200=--21");
}
