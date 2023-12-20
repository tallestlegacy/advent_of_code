use std::fs;

fn main() {
    // read file
    let file_name = "input.txt";
    let contents = fs::read_to_string(file_name).expect("Should have been able to read the file");

    // make file a vector
    let lines = contents.split("\n").collect::<Vec<&str>>();

    let mut sum = 0;

    for (idx, i) in lines.iter().enumerate() {
        // I kept getting an empty line at the end of the file
        if i.is_empty() {
            continue;
        }

        // check for 2-digit numbers and add them to the local nums
        let mut local_nums: Vec<u32> = Vec::new();
        for c in i.chars() {
            if is_number(c) {
                local_nums.push(c.to_digit(10).unwrap())
            }
        }

        // not possible based on the input, but just in case
        if local_nums.is_empty() {
            continue;
        }

        // get the 2-digit number
        let two_digit_num = local_nums[0] * 10 + local_nums[local_nums.len() - 1];
        println!("{idx}\t--->\t{two_digit_num}");

        // add to sum
        sum += two_digit_num;
    }

    println!("Sum\t--->\t{sum}");
}

// check if a char is a number
fn is_number(c: char) -> bool {
    let nums = String::from("0123456789");
    nums.contains(c)
}
