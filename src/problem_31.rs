pub fn get_problem() -> String {
    return "Problem 31: Coin sums".to_string();
}

pub fn solve_problem() -> String {
    let coins: Vec<i32> = vec![1, 2, 5, 10, 20, 50, 100, 200];
    let target_sum: i32 = 200;
    let solution: i32 = total_combinations(&coins, target_sum);

    return format!("{:?}", solution);
}

fn total_combinations(coins: &[i32], target_sum: i32) -> i32 {
    if coins.len() == 1 {
        return 1;
    }

    let mut total = 0;
    let mut most_significant_count: i32 = target_sum / coins[coins.len() - 1];

    if most_significant_count * coins[coins.len() - 1] == target_sum {
        total += 1;
        most_significant_count -= 1;
    }

    while most_significant_count >= 0 {
        let remaining_coins = &coins[0..(coins.len() - 1)];
        let remaining_sum = target_sum - (most_significant_count * coins[coins.len() - 1]);
        total += total_combinations(remaining_coins, remaining_sum);
        most_significant_count -= 1;
    }

    return total;
}