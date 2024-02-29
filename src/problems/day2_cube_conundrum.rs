use std::{fs, cmp::max};

const RED_COUNT: i32 = 12;
const GREEN_COUNT: i32 = 13;
const BLUE_COUNT: i32 = 14;

pub fn solve(){
    let input_string = fs::read_to_string("./input/day2_cube_conundrum.txt")
        .expect("Error in opening/reading file");

    let mut sum = 0;
    // Each game
    for line in input_string.lines() {
        let cur_id = parse_line_get_color_count(line);
        sum += cur_id;
    }

    println!("Sum of Valid Game Ids: {sum}");

}

pub fn solve2() {
    let input_string = fs::read_to_string("./input/day2_cube_conundrum.txt")
        .expect("Error in opening/reading file");

    let mut sum = 0;
    // Each game
    for line in input_string.lines() {
        let power = parse_line_get_minimum_color_power(line);
        sum += power;
    }
    println!("Sum of Power of minimum cubes: {sum}");
}


fn parse_line_get_color_count(line: &str) -> i32 {
    let (game_id_str, color_strs) = line.split_once(":").expect("expected format Game (id):");

    let (_, game_id_str) = game_id_str.split_once(" ").expect("Error in splitting game_id_str");
    let game_id: i32 = game_id_str.parse().expect("Error in parsing game_id");
    
    for s in color_strs.split(";") {
        let (red_count, green_count, blue_count) = parse_color_str(s);       
        if !is_valid_color_count(red_count, green_count, blue_count) {
            return 0;
        }
    }

    game_id
}

fn parse_line_get_minimum_color_power(line: &str) -> i32 {
    let (_game_id_str, color_strs) = line.split_once(":").expect("expected format Game (id):");

    let mut max_red = -1;
    let mut max_green = -1;
    let mut max_blue = -1;
    
    for s in color_strs.split(";") {
        let (red_count, green_count, blue_count) = parse_color_str(s);       
        max_red = max(red_count, max_red);
        max_green = max(green_count, max_green);
        max_blue = max(blue_count, max_blue);
    }
    max_red * max_green * max_blue
}

fn parse_color_str(color_str: &str) -> (i32, i32, i32){
    let mut red_count = 0;
    let mut green_count = 0;
    let mut blue_count = 0;

    for s in color_str.split(",") {
        let (count, color) = s.trim().split_once(" ").expect("expected format (count) (color)");
        let count: i32 = count.parse().expect("Expected integer");
        match color {
            "red" => red_count = count,
            "green" => green_count = count,
            "blue" => blue_count = count,
            _ => panic!("Invalid string in color")
        }
        
    }
    (red_count, green_count, blue_count)
}

fn is_valid_color_count(red: i32, green: i32, blue: i32) -> bool {
    if red <= RED_COUNT && green <= GREEN_COUNT && blue <= BLUE_COUNT {
        true
    }
    else{
        false
    }
}

