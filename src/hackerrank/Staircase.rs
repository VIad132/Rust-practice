use std::io;

pub fn staircase(n: usize) -> String {
    if n == 0 || n > 100 {
        return String::new();
    }

    let mut result = String::new();

    for i in 1..=n {
        result.push_str(&"#".repeat(i));
        result.push('\n');
    }

    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Input error");

    let n: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    print!("{}", staircase(n));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staircase_4() {
        let real = staircase(4);
        let expected = "\
#\n\
##\n\
###\n\
####\n";

        assert_eq!(real, expected);
    }

    #[test]
    fn test_invalid_input() {
        let real = staircase(0);
        let expected = "";
        assert_eq!(real, expected);
    }
}