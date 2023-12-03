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
