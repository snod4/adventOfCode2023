//Build iterator for use in char matrix for finding symbols
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
fn main() {
    let file: File = File::open("testInput.txt").expect("File open fail");

    let reader = BufReader::new(file);
    let char_matrix = get_char_matrix(reader);
    find_symbol_position(&char_matrix);
}

fn find_symbol_position(char_matrix: &CharMatrix) -> (usize, usize) {
    for i in 0..char_matrix.len() {
        println!("{},{}", i, char_matrix[i].len())
    }
    return (1, 1);
}

fn get_char_matrix(reader: BufReader<File>) -> Vec<Vec<char>> {
    let mut char_matrix: CharMatrix = Vec::new();
    for line in reader.lines() {
        let char_arr = line.unwrap().chars().collect();
        char_matrix.push(char_arr);
    }
    return char_matrix;
}
type CharMatrix = Vec<Vec<char>>;
