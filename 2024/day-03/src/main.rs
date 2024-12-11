use regex::Regex;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let pattern = Regex::new(r"((mul)(\()([0-9]+)(\,)([0-9]+)\))|(do\(\))|(don\'t\(\))").unwrap();

    let valid_operations: Vec<&str> = pattern.find_iter(INPUT).map(|m| m.as_str()).collect();

    println!("Part 1");
    println!(
        "Accumulator :: {}",
        run_equations(valid_operations.clone(), false)
    );

    println!("Part 2");
    println!("Accumulator :: {}", run_equations(valid_operations, true));
}

fn run_equations(equations: Vec<&str>, check_validate_flag: bool) -> i32 {
    let mut accumulator = 0;
    let mut is_enabled = true;

    for patter_match in equations {
        if patter_match.contains("don't()") {
            is_enabled = false;
        } else if patter_match.contains("do()") {
            is_enabled = true;
        } else if patter_match.contains("mul") && is_enabled || check_validate_flag == false {
            let tmp_pattern_match = patter_match
                .to_string()
                .replace("mul(", "")
                .replace(")", "");
            let numbers: Vec<i32> = tmp_pattern_match
                .split(",")
                .map(|num_str| num_str.parse::<i32>().expect(&patter_match))
                .collect();

            accumulator += numbers[0] * numbers[1];
        }
    }

    return accumulator;
}
