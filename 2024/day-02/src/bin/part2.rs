use std::usize;

const MIN_DIFFERENCE: i32 = 1;
const MAX_DIFFERENCE: i32 = 3;

fn main() {
    let input = include_str!("./input.txt");
    let mut reports: Vec<Vec<i32>> = Vec::new();
    let mut accumulator = 0;

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
    for (reportIndex, report) in reports.iter().enumerate() {
        match is_stable(report.to_vec()) {
            Ok(_) => accumulator += 1,

            Err(index) => {
                // try recovery
                let mut tmp_record_1 = report.clone();
                let mut tmp_record_2 = report.clone();
                tmp_record_2.remove(index);
                tmp_record_1.remove(index + 1);

                // recovery phase 1
                match is_stable(tmp_record_1.clone().to_vec()) {
                    Ok(_) => {
                        accumulator += 1;
                        println!("Phase 1");
                    }

                    // recovery phase 2
                    Err(_) => match is_stable(tmp_record_2.clone().to_vec()) {
                        Ok(_) => {
                            accumulator += 1;
                            println!("Phase 2");
                        }
                        Err(_) => {
                            println!("Line {}.\t\t{:?}", reportIndex + 1, report);
                        }
                    },
                }
            }
        }
    }

    println!("Accumulator : {accumulator}");
}

fn is_stable(report: Vec<i32>) -> Result<(), usize> {
    let is_increasing = report[0] < report[1];
    let mut stable = true;
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
