#![feature(let_chains)]

const ROOT: &str = env!("CARGO_MANIFEST_DIR");

fn get_input() -> (Vec<u32>, Vec<u32>) {
    let input = std::fs::read_to_string(format!("{ROOT}/input/day1.txt")).unwrap();

    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for line in input.lines() {
        let mut words = line.split_ascii_whitespace();
        left.push(words.next().unwrap().parse().unwrap());
        right.push(words.next().unwrap().parse().unwrap());
    }

    left.sort();
    right.sort();

    (left, right)
}

fn part1(left: &[u32], right: &[u32]) -> u32 {
    std::iter::zip(left, right)
        .map(|(l, r)| l.abs_diff(*r))
        .sum()
}

fn part2(left: &[u32], mut right: &[u32]) -> u32 {
    let mut score = 0;
    for l in left {
        while let [r, rest @ ..] = right
            && r <= l
        {
            if l == r {
                score += l;
            }
            right = rest;
        }
    }
    score
}

fn main() {
    let (left, right) = get_input();

    println!("part1: {}", part1(&left, &right));
    println!("part2: {}", part2(&left, &right));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let (left, right) = get_input();
        assert_eq!(super::part1(&left, &right), 1222801);
    }

    #[test]
    fn part2() {
        let (left, right) = get_input();
        assert_eq!(super::part2(&left, &right), 22545250);
    }
}
