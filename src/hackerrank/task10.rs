use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::HashMap;

pub fn sock_merchant(socks: &[i32]) -> i32 {
    let mut map = HashMap::new();

    for &sock in socks {
        *map.entry(sock).or_insert(0) += 1;
    }

    map.values().map(|count| count / 2).sum()
}

fn sockMerchant(_n: i32, ar: Vec<i32>) -> i32 {
    sock_merchant(&ar)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let result = sockMerchant(n, ar);

    writeln!(&mut fptr, "{}", result).ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case() {
        let socks = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];
        assert_eq!(sock_merchant(&socks), 3);
    }

    #[test]
    fn test_no_pairs() {
        let socks = vec![1, 2, 3, 4];
        assert_eq!(sock_merchant(&socks), 0);
    }

    #[test]
    fn test_all_pairs() {
        let socks = vec![1, 1, 2, 2, 3, 3];
        assert_eq!(sock_merchant(&socks), 3);
    }

    #[test]
    fn test_single_color() {
        let socks = vec![5, 5, 5, 5, 5];
        assert_eq!(sock_merchant(&socks), 2);
    }

    #[test]
    fn test_empty() {
        let socks: Vec<i32> = vec![];
        assert_eq!(sock_merchant(&socks), 0);
    }
}