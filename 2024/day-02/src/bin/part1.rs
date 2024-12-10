const MIN_DIFFERENCE: i32 = 1;
const MAX_DIFFERENCE: i32 = 3;

fn main() {
    let input = include_str!("./input.txt");
    let mut reports: Vec<Vec<i32>> = Vec::new();
    let mut accumulator = 0;
    let mut accumulator_neg = 0;

    for (index, line) in input.split("\n").into_iter().enumerate() {
        if line.is_empty() {
            continue;
        };

        // init each 1D vector
        reports.push(Vec::new());

        // fill the vector with the report values
        for value in line.split(" ") {
            match value.parse::<i32>() {
                Ok(value_int) => {
                    reports[index].push(value_int);
                }
                Err(e) => {
                    dbg!("{}", e);
                }
            }
        }
    }

    // analyze each line
    for (index, report) in reports.iter().enumerate() {
        let is_increasing = report[0] < report[1];
        let mut is_stable = true;

        // check the report values
        for i in 0..report.len() - 1 {
            let current_is_increasing = report[i] < report[i + 1];
            let diff = report[i] - report[i + 1];

            is_stable = is_increasing == current_is_increasing
                && diff.abs() >= MIN_DIFFERENCE
                && diff.abs() <= MAX_DIFFERENCE;

            if !is_stable {
                println!("{index}__{i} :: {diff}\t{current_is_increasing}:{is_increasing}",);
            }

            if !is_stable {
                accumulator_neg += 1;
                break;
            }
        }

        // track stable reports
        if is_stable {
            accumulator += 1;
        }
    }

    print!("Accumulator : {accumulator}::{accumulator_neg}");
}
