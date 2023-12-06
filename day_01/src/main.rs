use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Error reading puzzle input");

    part_02(input);
}

fn part_01(input: String) {
    let sum: u32 = input
        .lines()
        .map(|line| {
            let digits: Vec<char> = line
                .chars()
                .into_iter()
                .filter(|c| c.is_numeric())
                .collect();
            let line_calibration_num = format!("{}{}", digits[0], digits[digits.len() - 1])
                .parse::<u32>()
                .unwrap();
            //println!("{}", line_calibration_num);
            line_calibration_num
        })
        .sum();
    println!("Sum: {}", sum);
}

fn part_02(input: String) {
    let mut digit_strings: HashMap<&str, &str> = HashMap::new();
    digit_strings.insert("one", "on1e");
    digit_strings.insert("two", "tw2o");
    digit_strings.insert("three", "thr3ee");
    digit_strings.insert("four", "fo4ur");
    digit_strings.insert("five", "fi5ve");
    digit_strings.insert("six", "si6x");
    digit_strings.insert("seven", "sev7en");
    digit_strings.insert("eight", "eig8ht");
    digit_strings.insert("nine", "ni9ne");

    let sum: u32 = input
        .lines()
        .map(|line| {
            let mut modified_line: String = line.to_string();

            for (number_string, digit) in &digit_strings {
                modified_line = modified_line.replace(number_string, &format!("{}", digit));
            }

            let digits: Vec<char> = modified_line
                .chars()
                .into_iter()
                .filter(|c| c.is_numeric())
                .collect();
            let line_calibration_num = format!("{}{}", digits[0], digits[digits.len() - 1])
                .parse::<u32>()
                .unwrap();
           //  println!("{}", modified_line);
           //  println!("{}\n", line_calibration_num);
            line_calibration_num
        })
        .sum();
    println!("Sum: {}", sum);
}
