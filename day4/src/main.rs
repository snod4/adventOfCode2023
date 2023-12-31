use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file: File = File::open("input.txt").expect("File open fail");
    let reader = BufReader::new(file);
    // let mut sum: i32 = 0;
    //for line_result in reader.lines() {
    //    let mut matches = 0;
    //    let line = line_result.unwrap();
    //    let cardless_line = strip_card(&line);
    //    let mut sections = cardless_line.split('|'); let win_section = sections.next().unwrap(); let user_section = sections.next().unwrap(); let user_section_list: Vec<&str> = user_section.split_whitespace().collect();
    //    for win_val in win_section.split_whitespace() {
    //        if !user_section_list.contains(&win_val) {
    //            continue;
    //        }
    //        println!("Match val: {}", win_val);
    //        matches += 1;
    //    }
    //    if matches == 0 {
    //        continue;
    //    }
    //    sum += (2 as i32).pow(matches - 1);
    //    println!("Total matches: {}", matches);
    //}
    //println!("{}", sum)
    let mut card_data = create_card_data(reader);
    process_cards(&mut card_data);
    let total = calculate_total(card_data);
    println!("Total: {}", total)
}

fn create_card_data(reader: BufReader<File>) -> Vec<CardData> {
    let mut card_data: Vec<CardData> = Vec::new();
    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let id = get_card_id(&line);
        let cardless_line = strip_card(&line);
        let mut sections = cardless_line.split('|');
        let win_section = sections.next().unwrap();
        let user_section = sections.next().unwrap();
        let user_section_list: Vec<&str> = user_section.split_whitespace().collect();
        let mut matches = 0;
        for win_val in win_section.split_whitespace() {
            if !user_section_list.contains(&win_val) {
                continue;
            }
            matches += 1;
        }
        card_data.push(CardData::new(id, matches));
    }
    return card_data;
}
fn process_cards(card_list: &mut Vec<CardData>) {
    //println!("{}", card_list.len());
    for card_index in 0..card_list.len() {
        let card = &card_list[card_index];
        let match_count = card.match_count;
        let start = (card.id) as usize;
        let end = start + match_count;
        let num_of_times = card.copies;
        for i in start..end {
            card_list[i].copies += num_of_times;
        }
    }
}

fn calculate_total(card_list: Vec<CardData>) -> usize {
    let mut sum = 0;
    for card in card_list.iter() {
        sum += card.copies;
        //println!("{:?}", card);
    }
    return sum;
}
fn get_card_id(line: &str) -> i32 {
    let card_section = line.split(':').nth(0).unwrap();
    let id = card_section.split_whitespace().nth(1).unwrap();
    return id.parse().unwrap();
}

fn strip_card(line: &str) -> &str {
    let cardless_line = line.split(':').nth(1).unwrap();
    return cardless_line;
}
#[derive(Debug)]
struct CardData {
    id: i32,
    match_count: usize,
    copies: usize,
}
impl CardData {
    fn new(card_id: i32, match_count: usize) -> CardData {
        CardData {
            id: card_id,
            match_count,
            copies: 1,
        }
    }
}
