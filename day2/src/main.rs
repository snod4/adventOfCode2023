use std::{
    fs,
    io::{BufRead, BufReader},
};

fn main() {
    let file = fs::File::open("testInput.txt").expect("This failed");
    let reader = BufReader::new(file);

    for line_res in reader.lines() {
        let game_line = line_res.expect("Couldn't read line");
        let id = get_game_id(&game_line);
        println!("{}", id)
    }
    fn get_game_id(line: &str) -> u32 {
        let game: Vec<&str> = line.split(':').collect();
        let id: u32 = game[0]
            .split(' ')
            .nth(1)
            .expect("Unable to find ID")
            .parse()
            .expect("Unable to parse ID");
        return id;
    }
}
