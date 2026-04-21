use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'migratoryBirds' function below.
 */

pub fn migratory_birds(arr: &[i32]) -> i32 {
    let mut count = [0; 6];

    for &bird in arr {
        count[bird as usize] += 1;
    }

    let mut max_count = 0;
    let mut result = 1;

    for i in 1..=5 {
        if count[i] > max_count {
            max_count = count[i];
            result = i as i32;
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap())
        .expect("Failed to create output file");

    let _arr_count: i32 = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Invalid input");

    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Invalid number"))
        .collect();

    let result = migratory_birds(&arr);

    writeln!(fptr, "{}", result).expect("Write failed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        let arr = vec![1, 4, 4, 4, 5, 3];
        assert_eq!(migratory_birds(&arr), 4);
    }

    #[test]
    fn test_tie_case() {
        let arr = vec![1, 1, 2, 2, 3];
        assert_eq!(migratory_birds(&arr), 1);
    }

    #[test]
    fn test_single_element() {
        let arr = vec![3];
        assert_eq!(migratory_birds(&arr), 3);
    }

    #[test]
    fn test_all_same() {
        let arr = vec![2, 2, 2, 2];
        assert_eq!(migratory_birds(&arr), 2);
    }
}