use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

pub fn page_count(n: i32, p: i32) -> i32 {
    let from_front = p / 2;
    let from_back = (n - p) / 2;
    from_front.min(from_back)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let p = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = page_count(n, p);

    writeln!(&mut fptr, "{}", result).ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(page_count(6, 2), 1);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(page_count(5, 4), 0);
    }

    #[test]
    fn test_front_vs_back() {
        assert_eq!(page_count(10, 7), 1);
    }
}