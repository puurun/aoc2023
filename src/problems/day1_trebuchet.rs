use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn solve() {
    let str_path = "./input/day1_trebuchet.txt";
    let file_path = Path::new(str_path);
    let mut input_file =
        File::open(file_path).unwrap_or_else(|_| panic!("Open failed: {str_path}"));

    let mut buf = String::new();
    let _ = input_file
        .read_to_string(&mut buf)
        .unwrap_or_else(|_| panic!("Read failed: {str_path}"));

    // Can be shortened
    // let doc = fs::read_to_string("./input/day1_trebuchet.txt")
    //     .expect("Expects a string");

    let mut sum = 0;
    for line in buf.lines() {
        let num = extract_first_last_number(&line);
        sum += num;
    }
    println!("Sum of Calibration Values: {sum}");
}

fn extract_first_last_number(line: &str) -> i32 {
    // It turns out that regex usually doesn't support overlaps
    let search_str = line;

    let string_num_to_int = vec!["zero", "one", "two", "three",
        "four", "five", "six", "seven", "eight", "nine", 
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    
    let mut first_pos: i32 = -1;
    let mut first_num: i32 = -1;
    for (i, v) in string_num_to_int.iter().enumerate() {
        let cur_pos = search_str.find(v);
        if let Some(n) = cur_pos {
            let n = n as i32;
            let i = i as i32;
            if first_pos == -1 || n < first_pos {
                first_num = i%10;
                first_pos = n;
            }
        }
    }

    let mut last_pos: i32 = -1;
    let mut last_num: i32 = -1;
    for (i, v) in string_num_to_int.iter().enumerate() {
        let cur_pos = search_str.rfind(v);
        if let Some(n) = cur_pos {
            let n = n as i32;
            let i = i as i32;
            if last_pos == -1 || n > last_pos {
                last_num = i%10;
                last_pos = n;
            }
        }
    }

    first_num * 10 + last_num
}


