use std::fs;

const FILE_NAME: &str = "input.txt";

type Matrix = Vec<Vec<char>>;

fn main() {
    // read file
    let contents = fs::read_to_string(FILE_NAME).expect("Something went wrong reading the file");
    let lines = contents.lines().collect::<Vec<&str>>();

    // marshall
    let matrix = get_chars_matrix(lines);

    let mut sum = 0;

    for (index_y, line) in matrix.iter().enumerate() {
        let mut cached_number: i32 = 0;
        let mut is_valid = false;

        for (index_x, current_char) in line.iter().enumerate() {
            println!("({index_x}, {index_y}) = {current_char}, {cached_number}");
            // only test number characters in the matrix
            if !is_number(*current_char) {
                if cached_number > 0 {
                    println!("Cache: {cached_number}");
                }
                if is_valid {
                    sum += cached_number;
                }
                cached_number = 0;
                is_valid = false;
                continue;
            }

            cached_number = cached_number * 10 + current_char.to_digit(10).unwrap() as i32;

            // if we had a valid number before, we don't need to test further
            if is_valid {
                continue;
            }

            let surrounding_chars = parse_surrounding(&matrix, index_y as i32, index_x as i32);

            if has_special_characters(&surrounding_chars) {
                is_valid = true;
                println!("special character found ({index_x}, {index_y}) =  {current_char}, for cache {cached_number}");
            }

            // recording the result when the line is about to end
            if index_x == line.len() - 1 {
                println!("Cache: {cached_number}");
                if is_valid {
                    sum += cached_number;
                }
            }
        }
        // break;
    }
    println!("Sum : {sum}");
}

// transform lines to a matrix of characters
fn get_chars_matrix(lines: Vec<&str>) -> Matrix {
    let mut matrix: Matrix = Vec::new();

    for line in lines {
        matrix.push(line.chars().collect::<Vec<char>>());
    }

    matrix
}

// print all surrounding characters of a cell in the matrix
fn parse_surrounding(matrix: &Matrix, index_y: i32, index_x: i32) -> Vec<char> {
    let mut surrounding_chars: Vec<char> = Vec::new();

    let martix_width = matrix[0].len();
    let matrix_height = matrix.len();

    /*
     * [tr][tm][tl]
     * [ml]    [mr]
     * [bl][bm][br]
     */

    for i in 0..3 {
        let y: i32 = index_y + i - 1;

        // prevent overflow
        if y < 0 || y >= matrix_height as i32 {
            continue;
        }

        for j in 0..3 {
            let x: i32 = index_x + j - 1;

            // prevent overflow
            if x < 0 || x >= martix_width as i32 {
                continue;
            }
            // skip current cell
            if x == index_x && y == index_y {
                continue;
            }

            surrounding_chars.push(matrix[y as usize][x as usize]);
        }
    }

    surrounding_chars
}

fn is_number(c: char) -> bool {
    match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
        _ => false,
    }
}

fn has_special_characters(s: &Vec<char>) -> bool {
    for c in s.clone() {
        if !is_number(c) && c != '.' {
            return true;
        }
    }
    false
}
