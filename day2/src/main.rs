use std::{
    fs,
    io::{BufRead, BufReader},
};

fn main() {
    let file = fs::File::open("input.txt").expect("This failed");
    let reader = BufReader::new(file);
    let mut id_sum = 0;
    let mut power_sum = 0;

    for line_res in reader.lines() {
        let game_line = line_res.expect("Couldn't read line");
        let id = get_game_id(&game_line);
        //println!("{}", id);

        let info_line = get_info_section(&game_line);
        //println!("{}", info_line);

        let rounds = get_rounds(&info_line);
        //println!("{:?}", rounds);
        let mut pass = true;
        let mut psuedoGame: ImaginaryGame = ImaginaryGame {
            red: 0,
            blue: 0,
            green: 0,
        };
        for round in rounds.iter() {
            let marbles = get_marbles(&round);
            build_psuedo_game(&mut psuedoGame, &marbles)
            //println!("{:?}", marbles)
            //            pass = can_play(&marbles, 12, 13, 14);
            //            if !pass {
            //                break;
            //            }
        }
        power_sum += psuedoGame.blue * psuedoGame.green * psuedoGame.red;
        //if pass {
        //   id_sum += id;
        //}
    }
    println!("{}", power_sum)
}
fn build_psuedo_game(psuedoGame: &mut ImaginaryGame, marbles: &Vec<MarbleInfo>) {
    for marble in marbles.iter() {
        match marble.color {
            Color::RED => psuedoGame.red = std::cmp::max(psuedoGame.red, marble.count),
            Color::GREEN => psuedoGame.green = std::cmp::max(psuedoGame.green, marble.count),
            Color::BLUE => psuedoGame.blue = std::cmp::max(psuedoGame.blue, marble.count),
        }
    }
}

fn can_play(
    marbles: &Vec<MarbleInfo>,
    red_limit: usize,
    green_limit: usize,
    blue_limit: usize,
) -> bool {
    for marble in marbles.iter() {
        let pass = match marble.color {
            Color::RED => marble.count <= red_limit,
            Color::GREEN => marble.count <= green_limit,
            Color::BLUE => marble.count <= blue_limit,
        };
        if !pass {
            return false;
        }
    }
    return true;
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

fn get_info_section(line: &str) -> &str {
    let split_arr: Vec<&str> = line.split(':').collect();
    return split_arr[1];
}

fn get_rounds(info_line: &str) -> Vec<&str> {
    return info_line.split(';').map(|val: &str| val.trim()).collect();
}

fn get_marbles(round: &str) -> Vec<MarbleInfo> {
    let marble_iter = round.split(", ");
    let mut vec: Vec<MarbleInfo> = Vec::new();

    for marble_str in marble_iter {
        let marble_info_arr: Vec<&str> = marble_str.split(' ').collect();
        //println!("{:?}", marble_info_arr);
        vec.push(MarbleInfo {
            color: get_color(marble_info_arr[1]).unwrap(),
            count: marble_info_arr[0].parse().unwrap(),
        })
    }
    return vec;
}

fn get_color(color_str: &str) -> Result<Color, &str> {
    return match color_str {
        "red" => Ok(Color::RED),
        "green" => Ok(Color::GREEN),
        "blue" => Ok(Color::BLUE),
        _ => Err("color did not match"),
    };
}
#[derive(Debug)]
struct ImaginaryGame {
    red: usize,
    blue: usize,
    green: usize,
}

#[derive(Debug)]
enum Color {
    RED,
    BLUE,
    GREEN,
}

#[derive(Debug)]
struct MarbleInfo {
    color: Color,
    count: usize,
}
