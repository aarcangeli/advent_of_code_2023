use std::fs::read_to_string;

fn main() {
    println!("Counting numbers (day1)!");

    let filename = "input.txt";
    match read_to_string(filename) {
        Ok(file) => {
            part_two(file);
        }

        Err(msg) => {
            println!("ERROR: cannot read {filename} {msg}")
        }
    }
}

fn part_two(file: String) {
    let mut full_sum = 0;

    let numbers = vec![
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    for line in file.lines() {
        let mut first_digit: i32 = -1;
        let mut last_digit: i32 = -1;

        let mut first_digit_position: Option<usize> = None;
        let mut last_digit_position: Option<usize> = None;

        for (as_string, value) in numbers.iter() {
            match line.find(as_string) {
                None => {}
                Some(position) => {
                    if !first_digit_position.is_some() || position < first_digit_position.unwrap() {
                        first_digit_position = Some(position);
                        first_digit = value.clone();
                    }
                }
            }
            match line.rfind(as_string) {
                None => {}
                Some(position) => {
                    if !last_digit_position.is_some() || position > last_digit_position.unwrap() {
                        last_digit_position = Some(position);
                        last_digit = value.clone();
                    }
                }
            }
        }

        let full_num = first_digit * 10 + last_digit;
        full_sum = full_sum + full_num;
        println!("Line: {full_num}");
    }

    println!("Result: {full_sum}")
}
