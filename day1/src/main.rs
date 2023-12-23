use std::{
    fs,
    io::{BufRead, BufReader},
};

fn main() {
    let file = fs::File::open("inputTest.txt").expect("This failed");
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

fn get_number(val: &str) -> u32 {
    return match val.to_lowercase().as_str() {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    };
}

fn contains_number_word(val: &String, front: bool) -> Option<MyNumber> {
    let numbers_list = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut best_option: Option<MyNumber> = None;
    let mut best_loc = match front {
        true => usize::MAX,
        false => usize::MIN,
    };
    for num in numbers_list.iter() {
        let current_location = val.to_lowercase().find(num);
        if current_location.is_none() {
            continue;
        }
        let loc = current_location.unwrap();
        if front && loc < best_loc {
            best_loc = loc;
            best_option = Some(MyNumber {
                word: num.to_string(),
                location: loc,
                digit: get_number(num),
            });
        } else if !front && loc > best_loc {
            best_loc = loc;
            best_option = Some(MyNumber {
                word: num.to_string(),
                location: loc,
                digit: get_number(num),
            });
        }
    }
    return best_option;
}
#[derive(Debug)]
struct MyNumber {
    word: String,
    digit: u32,
    location: usize,
}

fn can_use_word(word_info: Option<&MyNumber>, index: usize) -> Option<char> {
    if word_info.is_none() {
        return None;
    }
    let word_val = word_info.unwrap();
    println!("{}", format!("{word_val:?}"));
    if index == word_val.location {
        return char::from_digit(word_val.digit, 10);
    } else {
        return None;
    }
}

fn get_next_digit(val: &String) -> Option<char> {
    let word_info = contains_number_word(val, true);
    let mut count = 0;
    for letter in val.chars() {
        if let Some(val) = can_use_word(word_info.as_ref(), count) {
            return Some(val);
        }
        if letter.is_digit(10) {
            return Some(letter);
        }
        count += 1;
    }
    return None;
}

fn get_last_digit(val: &String) -> Option<char> {
    let word_info = contains_number_word(val, false);
    let mut count = val.chars().count() - 1;
    for letter in val.chars().rev() {
        if let Some(val) = can_use_word(word_info.as_ref(), count) {
            return Some(val);
        }
        if letter.is_digit(10) {
            return Some(letter);
        }
        count -= 1;
    }
    return None;
}
