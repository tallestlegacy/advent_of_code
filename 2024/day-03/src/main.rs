use regex::Regex;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut accumulator = 0;

    let pattern = Regex::new(r"((mul)(\()([0-9]+)(\,)([0-9]+)\))").unwrap();

    for patter_match in pattern.find_iter(INPUT).map(|m| m.as_str()) {
        let tmp_pattern_match = patter_match
            .to_string()
            .replace("mul(", "")
            .replace(")", "");
        let numbers: Vec<i32> = tmp_pattern_match
            .split(",")
            .map(|num_str| num_str.parse::<i32>().expect(patter_match))
            .collect();

        println!("pattern {:?}\t\tnumbers {:?}", patter_match, numbers);

        accumulator += numbers[0] * numbers[1];
    }

    println!("Accumulator :: {accumulator}");
}
