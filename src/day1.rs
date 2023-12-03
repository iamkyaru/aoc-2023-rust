use aoc_runner_derive::aoc;

fn is_calibration_value(c: &char) -> bool {
    c.is_ascii_digit()
}

#[aoc(day1, part1)]
pub fn solve1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|s| {
            s.chars()
                .find(is_calibration_value)
                .zip(s.chars().rfind(is_calibration_value))
        })
        .filter_map(|(left, right)| left.to_digit(10).zip(right.to_digit(10)))
        .map(|(left, right)| (left * 10 + right) as usize)
        .sum()
}

#[aoc(day1, part2)]
pub fn solve2(input: &str) -> usize {
    let mut calibration_value = 0;

    for line in input.lines() {
        calibration_value += new_algo(line);
    }

    calibration_value
}

const WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn new_algo(line: &str) -> usize {
    let mut line = line.to_string();

    WORDS.into_iter().for_each(|word| loop {
        let idx = match line.find(word) {
            Some(idx) => idx,
            _ => break,
        };

        let digit_char = word_to_digit(word)
            .map(|digit| digit as u32)
            .and_then(|digit| char::from_digit(digit, 10))
            .unwrap();
        line.insert(idx + 1, digit_char);
    });

    line.chars()
        .find(is_calibration_value)
        .zip(line.chars().rfind(is_calibration_value))
        .and_then(|(left, right)| left.to_digit(10).zip(right.to_digit(10)))
        .map(|(left, right)| left * 10 + right)
        .unwrap() as usize
}

fn word_to_digit(word: &str) -> Option<usize> {
    match word {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}
