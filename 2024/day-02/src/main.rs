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
    for (_index, report) in reports.iter().enumerate() {
        match is_stable(report.to_vec()) {
            Ok(_) => accumulator += 1,
            Err(_) => accumulator_neg += 1,
        }
    }

    println!("Part 1");
    println!("Accumulator : {accumulator}::{accumulator_neg}");

    // recursive checks : honestly not my best work but it works
    for (_index, report) in reports.iter().enumerate() {
        match is_stable(report.to_vec()) {
            Ok(_) => {}
            Err(_) => {
                // check every index
                for i in 0..report.len() {
                    let mut recovery_array = report.clone();
                    recovery_array.remove(i);
                    match is_stable(recovery_array) {
                        Ok(_) => {
                            accumulator += 1;
                            break;
                        }
                        Err(_) => {}
                    }
                }
            }
        }
    }

    println!("Part 2");
    println!("Accumulator : {accumulator}");
}

fn is_stable(report: Vec<i32>) -> Result<(), usize> {
    let is_increasing = report[0] < report[1];
    let mut stable: bool;
    let mut i: usize = 0;

    // check the report values
    while i < report.len() - 1 {
        let current_is_increasing = report[i] < report[i + 1];
        let diff = report[i] - report[i + 1];

        stable = is_increasing == current_is_increasing
            && diff.abs() >= MIN_DIFFERENCE
            && diff.abs() <= MAX_DIFFERENCE;

        if !stable {
            return Err(i);
        }
        i += 1;
    }

    return Ok(());
}
