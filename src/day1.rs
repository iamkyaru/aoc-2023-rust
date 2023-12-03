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
        calibration_value += solve_mixed(line).unwrap_or(0);
    }

    calibration_value
}

fn solve_mixed(line: &str) -> Option<usize> {
    let mut value = 0;

    let mut buffer = Vec::new();

    value += algorithm(line.chars(), &mut buffer)? * 10;
    value += algorithm(line.chars().rev(), &mut buffer)?;

    Some(value)
}

fn algorithm<I: Iterator<Item = char>>(mut iter: I, buffer: &mut Vec<char>) -> Option<usize> {
    loop {
        if let Some(c) = iter.next() {
            if buffer.len() >= 3 {
                let word = buffer.drain(..).collect::<String>();
            }

            if c.is_ascii_digit() {
                let word = buffer.drain(..).collect::<String>();

                break word_to_digit(&word).or_else(|| c.to_digit(10).map(|digit| digit as usize));
            }

            buffer.push(c);
        }
    }
}

fn new_algo() {
    todo!()
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
