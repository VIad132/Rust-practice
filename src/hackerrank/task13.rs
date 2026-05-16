use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn divisible_sum_pairs(n: i32, k: i32, ar: &[i32]) -> i32 {
    let mut count = 0;

    for i in 0..n as usize {
        for j in (i + 1)..n as usize {
            if (ar[i] + ar[j]) % k == 0 {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0]
        .trim()
        .parse::<i32>()
        .unwrap();

    let k = first_multiple_input[1]
        .trim()
        .parse::<i32>()
        .unwrap();

    let ar: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let result = divisible_sum_pairs(n, k, &ar);

    writeln!(&mut fptr, "{}", result).ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisible_sum_pairs() {
        let n = 6;
        let k = 3;
        let ar = vec![1, 3, 2, 6, 1, 2];

        assert_eq!(divisible_sum_pairs(n, k, &ar), 5);
    }

      #[test]
fn test_no_pairs() {
    let n = 4;
    let k = 5;
    let ar = vec![1, 1, 1, 1];

    assert_eq!(divisible_sum_pairs(n, k, &ar), 0);
}

    #[test]
    fn test_all_pairs() {
        let n = 4;
        let k = 2;
        let ar = vec![2, 2, 2, 2];

        assert_eq!(divisible_sum_pairs(n, k, &ar), 6);
    }
}