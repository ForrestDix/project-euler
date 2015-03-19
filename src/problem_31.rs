pub fn get_problem() -> String {
    return "Problem 31: Coin sums".to_string();
}

pub fn solve_problem() -> String {
    let coins: Vec<u32> = vec![1, 2, 5, 10, 20, 50, 100, 200];
    let mut max_coins: Vec<u32> = Vec::new();
    let target_sum: u32 = 200;
    let mut total_valid_combinations: u32 = 0;

    // Find the maximum number of each coin type.
    for n in 0..coins.len() {
        max_coins.push(target_sum/coins[n]);
    }

    loop {
        // Use greedy algorithm within the confines of max_coins.
        
        break;
    }

    let solution = format!("{:?}", max_coins);//"solution".to_string();
    return solution;
}

fn greedy_sum_possible(coins: Vec<u32>, max_coins: Vec<u32>, target_sum: u32) -> bool {
    loop {
        let mut total = 0;
        let mut current_coin = coins.len() - 1;
        break;
    }

    return false;
}