use std::iter::Enumerate;

fn main() {
    let input = include_str!("./input.txt");

    let mut left_inputs: Vec<i32> = Vec::new();
    let mut right_inputs: Vec<i32> = Vec::new();
    let mut accumulator = 0;

    // read all line inputs
    for line in input.split("\n") {
        let nums = line.split("   ").collect::<Vec<&str>>();

        match nums[0].parse::<i32>() {
            Ok(_) => {
                left_inputs.push(nums[0].parse::<i32>().unwrap());
                right_inputs.push(nums[1].parse::<i32>().unwrap());
            }
            Err(_) => {
                dbg!("{}", nums);
            }
        }
    }

    // sort both lists
    left_inputs.sort();
    right_inputs.sort();

    // get unique left values
    let mut unique_left_inputs: Vec<i32> = Vec::new();
    for i in left_inputs {
        if unique_left_inputs.contains(&i) {
            continue;
        } else {
            unique_left_inputs.push(i);
        }
    }

    // find the frequency of each

    for number in unique_left_inputs {
        let mut frequency = 0;
        for check_number in right_inputs.iter() {
            if number == *check_number {
                frequency += 1
            }
        }

        accumulator += frequency * number
    }

    print!("Accumulator : {accumulator}");
}
