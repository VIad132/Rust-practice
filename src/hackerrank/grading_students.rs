use std::io::{self, BufRead};

fn grading_students(grades: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(grades.len());

    for &grade in grades {
        if grade < 38 {
            result.push(grade);
        } else {
            let next_multiple = ((grade + 4) / 5) * 5;

            if next_multiple - grade < 3 {
                result.push(next_multiple);
            } else {
                result.push(grade);
            }
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut grades = Vec::with_capacity(n);

    for _ in 0..n {
        let grade: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        grades.push(grade);
    }

    let result = grading_students(&grades);

    for value in result {
        println!("{}", value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grading_students() {
        let grades = vec![73, 67, 38, 33];
        let result = grading_students(&grades);

        assert_eq!(result, vec![75, 67, 40, 33]);
    }
}