use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'breakingRecords' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY scores as parameter.
 */

pub fn breaking_records(scores: &[i32]) -> (i32, i32) {
    if scores.is_empty() {
        return (0, 0);
    }

    let mut max_score = scores[0];
    let mut min_score = scores[0];

    let mut max_breaks = 0;
    let mut min_breaks = 0;

    for &score in &scores[1..] {
        if score > max_score {
            max_score = score;
            max_breaks += 1;
        } else if score < min_score {
            min_score = score;
            min_breaks += 1;
        }
    }

    (max_breaks, min_breaks)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let scores: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let result = breaking_records(&scores);

    write!(&mut fptr, "{} {}", result.0, result.1).unwrap();
    writeln!(&mut fptr).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case() {
        let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
        let result = breaking_records(&scores);
        assert_eq!(result, (2, 4));
    }

    #[test]
    fn test_no_breaks() {
        let scores = vec![5, 5, 5, 5];
        let result = breaking_records(&scores);
        assert_eq!(result, (0, 0));
    }

    #[test]
    fn test_increasing() {
        let scores = vec![1, 2, 3, 4, 5];
        let result = breaking_records(&scores);
        assert_eq!(result, (4, 0));
    }

    #[test]
    fn test_decreasing() {
        let scores = vec![5, 4, 3, 2, 1];
        let result = breaking_records(&scores);
        assert_eq!(result, (0, 4));
    }

    #[test]
    fn test_single_element() {
        let scores = vec![10];
        let result = breaking_records(&scores);
        assert_eq!(result, (0, 0));
    }
}