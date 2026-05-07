use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut primary = 0;
    let mut secondary = 0;

    for i in 0..n {
        primary += arr[i][i];
        secondary += arr[i][n - 1 - i];
    }

    (primary - secondary).abs()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n);

    for _ in 0..n {
        let row: Vec<i32> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        arr.push(row);
    }

    let result = diagonal_difference(&arr);

    writeln!(&mut fptr, "{}", result).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagonal_difference() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![9, 8, 9],
        ];

        assert_eq!(diagonal_difference(&matrix), 2);
    }
}