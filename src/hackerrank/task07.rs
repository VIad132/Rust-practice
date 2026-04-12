use std::io;

pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    fn gcd(mut x: i32, mut y: i32) -> i32 {
        while y != 0 {
            let temp = y;
            y = x % y;
            x = temp;
        }
        x
    }

    fn lcm(x: i32, y: i32) -> i32 {
        x * y / gcd(x, y)
    }

    let lcm_a = a.iter().fold(a[0], |acc, &x| lcm(acc, x));
    let gcd_b = b.iter().fold(b[0], |acc, &x| gcd(acc, x));

    let mut count = 0;
    let mut multiple = lcm_a;

    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }

    count
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let a: Vec<i32> = input.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let b: Vec<i32> = input.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = get_total_x(&a, &b);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_x() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];
        assert_eq!(get_total_x(&a, &b), 3);
    }

    #[test]
    fn test_single_element() {
        let a = vec![1];
        let b = vec![100];
        assert_eq!(get_total_x(&a, &b), 9);
    }
}