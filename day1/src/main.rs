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

fn part_one(file: String) {
    let mut full_sum = 0;

    for line in file.lines() {
        let mut first_digit: i32 = -1;
        let mut last_digit: i32 = -1;

        for char in line.chars() {
            match char.to_string().parse::<i32>() {
                Ok(num) => {
                    first_digit = num;
                    break;
                }
                Err(_) => {}
            }
        }

        for char in line.chars() {
            match char.to_string().parse::<i32>() {
                Ok(num) => {
                    last_digit = num;
                }
                Err(_) => {}
            }
        }

        let full_num = first_digit * 10 + last_digit;
        full_sum = full_sum + full_num;
        // println!("Line: {full_num}");
    }

    println!("Result: {full_sum}")
}

fn part_two(file: String) {
    let mut full_sum = 0;

    for mut line in file.lines() {
        let mut first_digit: i32 = -1;
        let mut last_digit: i32 = -1;

        let line2 = line
            .replace("one", "1")
            .replace("two", "2")
            .replace("three", "3")
            .replace("four", "4")
            .replace("five", "5")
            .replace("six", "6")
            .replace("seven", "7")
            .replace("eight", "8")
            .replace("nine", "9");

        for char in line2.chars() {
            match char.to_string().parse::<i32>() {
                Ok(num) => {
                    first_digit = num;
                    break;
                }
                Err(_) => {}
            }
        }

        for char in line2.chars() {
            match char.to_string().parse::<i32>() {
                Ok(num) => {
                    last_digit = num;
                }
                Err(_) => {}
            }
        }

        let full_num = first_digit * 10 + last_digit;
        full_sum = full_sum + full_num;
        println!("Line: {full_num}");
    }

    println!("Result: {full_sum}")
}
