fn sum_abs_diff(xs: &[u32], ys: &[u32]) -> u32 {
    std::iter::zip(xs, ys).map(|(l, r)| l.abs_diff(*r)).sum()
}

const ROOT: &str = env!("CARGO_MANIFEST_DIR");

fn main() {
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

    let answer = sum_abs_diff(&left, &right);
    println!("{answer}");
}
