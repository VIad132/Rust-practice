use std::io::{self, BufRead};

fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let apple_count = apples.iter()
        .map(|&d| a + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count();

    let orange_count = oranges.iter()
        .map(|&d| b + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count();

    println!("{}", apple_count);
    println!("{}", orange_count);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let second: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let _third: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let apples: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let oranges: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    count_apples_and_oranges(first[0], first[1], second[0], second[1], &apples, &oranges);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_case() {
        let s = 7;
        let t = 11;
        let a = 5;
        let b = 15;

        let apples = vec![-2, 2, 1];
        let oranges = vec![5, -6];

        let apple_count = apples.iter()
            .map(|&d| a + d)
            .filter(|&pos| pos >= s && pos <= t)
            .count();

        let orange_count = oranges.iter()
            .map(|&d| b + d)
            .filter(|&pos| pos >= s && pos <= t)
            .count();

        assert_eq!(apple_count, 1);
        assert_eq!(orange_count, 1);
    }

    #[test]
    fn test_no_fruits() {
        let s = 0;
        let t = 5;
        let a = -10;
        let b = 10;

        let apples = vec![-1, -2];
        let oranges = vec![1, 2];

        let apple_count = apples.iter()
            .map(|&d| a + d)
            .filter(|&pos| pos >= s && pos <= t)
            .count();

        let orange_count = oranges.iter()
            .map(|&d| b + d)
            .filter(|&pos| pos >= s && pos <= t)
            .count();

        assert_eq!(apple_count, 0);
        assert_eq!(orange_count, 0);
    }
}