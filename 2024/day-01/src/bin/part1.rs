fn main() {
    let input = include_str!("./input1.txt");

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

    // sort the lists
    left_inputs.sort();
    right_inputs.sort();

    // find the difference
    for i in 0..left_inputs.len() {
        accumulator += (left_inputs[i] - right_inputs[i]).abs()
    }

    print!("Accumulator : {accumulator}");
}
