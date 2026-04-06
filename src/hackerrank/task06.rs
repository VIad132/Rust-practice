use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'kangaroo' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. INTEGER x1
 *  2. INTEGER v1
 *  3. INTEGER x2
 *  4. INTEGER v2
 */

fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 == v2 {
        return if x1 == x2 {
            "YES".to_string()
        } else {
            "NO".to_string()
        };
    }

    let numerator = x2 - x1;
    let denominator = v1 - v2;

    if denominator != 0 && numerator % denominator == 0 && numerator / denominator >= 0 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let x1 = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let v1 = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let x2 = first_multiple_input[2].trim().parse::<i32>().unwrap();

    let v2 = first_multiple_input[3].trim().parse::<i32>().unwrap();

    let result = kangaroo(x1, v1, x2, v2);

    writeln!(&mut fptr, "{}", result).ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kangaroo_yes() {
        assert_eq!(kangaroo(0, 3, 4, 2), "YES");
    }

    #[test]
    fn test_kangaroo_no() {
        assert_eq!(kangaroo(0, 2, 5, 3), "NO");
    }
}