pub fn get_problem() -> String {
    return "Problem 48: Self powers".to_string();
}

pub fn solve_problem() -> String {
    let highest_power: u64 = 1000;
    let mut current_power: u64 = highest_power;
    let mut last_ten_digits: u64 = 0;

    while current_power > 0 {
        last_ten_digits = truncate(last_ten_digits + power(current_power));
        current_power -= 1;
    }

    return format!("{:?}", last_ten_digits);
}

fn power(power: u64) -> u64 {
    let mut product: u64 = power;

    for n in 1 .. power {
        product = truncate(product * power);
    }

    return product;
}

fn truncate(value: u64) -> u64 {
    let mut truncated_value = value;

    while truncated_value > 9999999999 {
        truncated_value -= 10000000000;
    }

    return truncated_value;
}