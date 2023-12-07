use std::{
    fs,
    io::{BufRead, BufReader},
};
fn main() {
    let file = fs::File::open("input.txt").expect("This failed");
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line_res in reader.lines() {
        let line = line_res.unwrap();
        let first_digit = match get_next_digit(&line) {
            Some(val) => val,
            None => continue,
        };
        let last_digit = match get_last_digit(&line) {
            Some(val) => val,
            None => continue,
        };
        let mut string_val = String::new();
        string_val.push(first_digit);
        string_val.push(last_digit);
        println!("{}{}", first_digit, last_digit);
        sum += string_val.as_str().parse::<i32>().unwrap();
    }

    println!("{}", sum);
}

is_word(val:&str)-> {
    //implment word number match and convert to number
}

fn get_next_digit(val: &String) -> Option<char> {
    for letter in val.chars() {
        if letter.is_digit(10) {
            return Some(letter);
        }
    }
    return None;
}

fn get_last_digit(val: &String) -> Option<char> {
    for letter in val.chars().rev() {
        if letter.is_digit(10) {
            return Some(letter);
        }
    }
    return None;
}
