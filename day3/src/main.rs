//Build iterator for use in char matrix for finding symbols
//Figure out how to make iterator and still access other data (method?)
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
fn main() {
    let file: File = File::open("input.txt").expect("File open fail");

    let reader = BufReader::new(file);
    let mut sum = 0;
    let mut char_matrix = get_char_matrix(reader);
    let iter: Vec<(usize, usize)> = char_matrix.iter().collect();
    println!("{:?}", iter);
    for symbol_location in iter {
        println!("{:?}", symbol_location);
        println!(
            "{}",
            char_matrix.get_val(symbol_location.0, symbol_location.1)
        );

        //sum += char_matrix.find_numbers_near_symbol(symbol_location.0, symbol_location.1);
        sum += char_matrix.find_numbers_part2_near_symbol(symbol_location.0, symbol_location.1);
    }
    println!("Sum is {}", sum)
}

fn get_char_matrix(reader: BufReader<File>) -> CharMatrix {
    let mut char_matrix: CharMatrix = CharMatrix::new();
    for line in reader.lines() {
        let char_arr = line.unwrap().chars().collect();
        char_matrix.data.push(char_arr);
    }
    return char_matrix;
}

//type CharMatrix = Vec<Vec<char>>;

struct CharMatrix {
    data: Vec<Vec<char>>,
    visited_locations: Vec<(usize, usize)>,
}
impl CharMatrix {
    fn new() -> CharMatrix {
        CharMatrix {
            data: Vec::new(),
            visited_locations: Vec::new(),
        }
    }

    fn iter(&self) -> CharMatrixIter {
        return CharMatrixIter {
            last_location: (0, 0),
            data: &(self.data),
        };
    }

    fn get_val(&self, row: usize, col: usize) -> char {
        return self.data[row][col];
    }

    fn find_numbers_part1_near_symbol(&mut self, row: usize, col: usize) -> i32 {
        if !is_symbol(self.get_val(row, col)) {
            panic!("The provided row and column does not contain a symbol");
        }
        let mut sum = 0;
        for row_offset in -1..2 {
            let cur_row = row as i32 + row_offset;
            if cur_row < 0 || cur_row >= self.data.len().try_into().unwrap() {
                continue;
            }
            for col_offset in -1..2 {
                let cur_col = col as i32 + col_offset;
                //If we've already checked this location for a number i.e. when searching for the
                //rest of a number, don't bother again
                if self
                    .visited_locations
                    .contains(&(cur_row as usize, cur_col as usize))
                {
                    continue;
                }
                if cur_col < 0 || cur_col >= self.data[cur_row as usize].len() as i32 {
                    continue;
                }
                if (row_offset, col_offset) == (0, 0) {
                    continue;
                }
                let character = self.data[cur_row as usize][cur_col as usize];
                if !char::is_digit(character, 10) {
                    continue;
                };
                println!(
                    "At {},{} found {}",
                    cur_row, cur_col, self.data[cur_row as usize][cur_col as usize]
                );
                let val = self.get_from_both(cur_row as usize, cur_col as usize);
                println!("Found whole number: {}", val);
                sum += val;
            }
        }
        return sum;
    }

    fn find_numbers_part2_near_symbol(&mut self, row: usize, col: usize) -> i32 {
        if !is_gear(self.get_val(row, col)) {
            return 0;
        }
        let mut product = 1;
        let mut count = 0;
        for row_offset in -1..2 {
            let cur_row = row as i32 + row_offset;
            if cur_row < 0 || cur_row >= self.data.len().try_into().unwrap() {
                continue;
            }
            for col_offset in -1..2 {
                let cur_col = col as i32 + col_offset;

                //If we've already checked this location for a number i.e. when searching for the
                //rest of a number, don't bother again
                if self
                    .visited_locations
                    .contains(&(cur_row as usize, cur_col as usize))
                {
                    continue;
                }
                if cur_col < 0 || cur_col >= self.data[cur_row as usize].len() as i32 {
                    continue;
                }
                if (row_offset, col_offset) == (0, 0) {
                    continue;
                }
                let character = self.data[cur_row as usize][cur_col as usize];
                if !char::is_digit(character, 10) {
                    continue;
                };
                println!(
                    "At {},{} found {}",
                    cur_row, cur_col, self.data[cur_row as usize][cur_col as usize]
                );
                let val = self.get_from_both(cur_row as usize, cur_col as usize);
                count += 1;
                println!("Found whole number: {}", val);
                product *= val;
            }
        }
        if count != 2 {
            return 0;
        }
        return product;
    }
    fn get_from_both(&mut self, row: usize, start_col: usize) -> i32 {
        // Go outward from center and stop on each side individually
        let mut left_stop = false;
        let mut right_stop = false;
        let mut offset = 0;
        let mut number = String::from(self.data[row][start_col]);
        while !left_stop || !right_stop {
            //println!("{},{}", left_stop, right_stop);
            offset += 1;
            let left_move = start_col as i32 - offset; //Use i32 cast becusae it can be negative
            let right_move = start_col + offset as usize;
            if !left_stop {
                if left_move < 0 {
                    left_stop = true;
                } else {
                    let left_char = self.data[row][left_move as usize];
                    if char::is_digit(left_char, 10) {
                        number.insert(0, left_char);
                        self.visited_locations.push((row, left_move as usize))
                    } else {
                        left_stop = true;
                    }
                }
            }
            //Go other direction
            if !right_stop {
                if right_move >= self.data[row].len() {
                    right_stop = true;
                } else {
                    let right_char = self.data[row][right_move];
                    if char::is_digit(right_char, 10) {
                        number.push(right_char);
                        self.visited_locations.push((row, right_move))
                    } else {
                        right_stop = true;
                    }
                }
            }
        }
        if number == "" {
            panic!("We should have a number by now")
        }
        return number.parse::<i32>().unwrap();
    }
}

enum Direction {
    Right,
    Left,
    Both,
}

fn is_symbol(letter: char) -> bool {
    return !(letter.is_digit(10) || letter == '.');
}
fn is_gear(letter: char) -> bool {
    return letter == '*';
}
struct CharMatrixIter<'a> {
    last_location: (usize, usize),
    data: &'a Vec<Vec<char>>,
}
impl Iterator for CharMatrixIter<'_> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        for row_index in self.last_location.0..self.data.len() {
            let row = &self.data[row_index];
            let mut col_start = 0;
            if row_index == self.last_location.0 {
                col_start = self.last_location.1 + 1;
            }
            for col_index in col_start..row.len() {
                let letter = row[col_index];

                if !is_symbol(letter) {
                    continue;
                }
                self.last_location = (row_index, col_index);
                return Some((row_index, col_index));
            }
        }
        return None;
    }
}
