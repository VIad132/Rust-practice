pub fn bon_appetit_result(bill: &[i32], skipped_index: usize, charged: i32) -> String {
    let actual_share: i32 = bill
        .iter()
        .enumerate()
        .filter_map(|(i, cost)| (i != skipped_index).then_some(*cost))
        .sum::<i32>()
        / 2;

    if charged == actual_share {
        "Bon Appetit".to_string()
    } else {
        (charged - actual_share).to_string()
    }
}

pub fn solve(input: &str) -> String {
    let mut values = input.split_whitespace();

    let item_count = values.next().unwrap().parse::<usize>().unwrap();
    let skipped_index = values.next().unwrap().parse::<usize>().unwrap();

    let bill: Vec<i32> = (0..item_count)
        .map(|_| values.next().unwrap().parse::<i32>().unwrap())
        .collect();

    let charged = values.next().unwrap().parse::<i32>().unwrap();

    bon_appetit_result(&bill, skipped_index, charged)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overcharged_case() {
        assert_eq!(bon_appetit_result(&[3, 10, 2, 9], 1, 12), "5");
    }

    #[test]
    fn fair_split() {
        assert_eq!(bon_appetit_result(&[3, 10, 2, 9], 1, 7), "Bon Appetit");
    }

    #[test]
    fn full_input() {
        assert_eq!(solve("4 1\n3 10 2 9\n12\n"), "5");
    }
}