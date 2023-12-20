use std::fs;

const FILE_NAME: &str = "input.txt";
const ALPHABET_NUMERICS: [&str; 20] = [
    // words
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    // numbers
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
];
fn main() {
    // read file
    let contents = fs::read_to_string(FILE_NAME).expect("Should have been able to read the file");

    // get each line
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let mut sum = 0;

    for (index, line) in lines.into_iter().enumerate() {
        if line.is_empty() {
            continue;
        }

        let num = get_num_from_line(line);
        println!("{}\t--->\t{}", index + 1, num);
        sum += num;
    }

    println!("sum\t==>\t{}", sum);
}

fn get_num_from_line(line: &str) -> u32 {
    let mut first_num = 0;
    let mut second_num = 0;

    let mut memo;

    // going forward
    memo = String::new();
    for i in line.chars() {
        memo.push(i);
        let n = find_num_in_string(memo.as_str());
        if n != 0 {
            first_num = n;
            break;
        }
    }

    // going in reverse
    memo = String::new();
    for i in line.chars().rev() {
        memo.push(i);
        let n = find_num_in_string(memo.as_str().chars().rev().collect::<String>().as_str());
        if n != 0 {
            second_num = n;
            break;
        }
    }

    first_num * 10 + second_num
}

fn find_num_in_string(line: &str) -> u32 {
    for i in ALPHABET_NUMERICS {
        if line.contains(i) {
            return to_number(i);
        }
    }
    0
}

fn to_number(s: &str) -> u32 {
    match s {
        "0" | "zero" => 0,
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => 0,
    }
}
