use std::{collections::HashSet, fmt, fs};

#[derive(Eq, Hash, PartialEq)]
struct Number {
    value: i32,
    position: Vec<(u32, u32)>,
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Number")
            .field("value", &self.value)
            .field("position", &self.position)
            .finish()
    }
}

struct Symbol {
    value: char,
    position: (u32, u32),
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Symbol")
            .field("value", &self.value)
            .field("position", &self.position)
            .finish()
    }
}

pub fn solve() {
    let input =
        fs::read_to_string("input/day3_gear_ratios.txt").expect("Error in opening/reading file");

    let (mut number_hash, symbol_vec) = parse_input(&input);

    let mut sum = 0;

    number_hash.retain(|number| {
        let mut number_delete = false;

        for symbol in symbol_vec.iter() {
            let (sy, sx) = symbol.position;
            for &(y, x) in number.position.iter() {
                if (sy - 1 <= y && y <= sy + 1) && (sx - 1 <= x && x <= sx + 1) {
                    sum += number.value;
                    number_delete = true;
                    break;
                }
            }
            if number_delete {
                break;
            }
        }

        if number_delete {
            return false;
        }

        true
    });

    println!("Sum of Engine Part Numbers: {sum}");
}

pub fn solve2() {
    let input =
        fs::read_to_string("input/day3_gear_ratios.txt").expect("Error in opening/reading file");

    let (number_hash, symbol_vec) = parse_input(&input);

    let mut gear_ratio: i64 = 0;

    for symbol in symbol_vec.iter() {
        if symbol.value != '*' {
            continue;
        }
        let (sy, sx) = symbol.position;
        let mut number_count = 0;
        let mut mul: i64 = 1;
        for num in number_hash.iter() {
            let mut is_adjacent = false;
            for &(y, x) in num.position.iter() {
                if sy-1 <= y && y <= sy+1 && sx-1 <= x && x <= sx+1 {
                    is_adjacent = true;
                }
            }

            if is_adjacent {
                number_count += 1;
                if number_count > 2 {
                    break;
                }
                mul *= num.value as i64;
            }
        }

        if number_count == 2 {
            gear_ratio += mul;
        }

    }

    println!("Sum of Gear Ratios: {gear_ratio}");

}

// returns hashset of numbers, and hashset of symbols
fn parse_input(input: &str) -> (HashSet<Number>, Vec<Symbol>) {
    let mut number_hash: HashSet<Number> = HashSet::new();
    let mut symbol_vec: Vec<Symbol> = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let mut is_number = false;
        let mut start_pos_number = 0;

        for (j, c) in line.char_indices() {
            if c.is_ascii_digit() {
                if !is_number {
                    is_number = true;
                    start_pos_number = j;
                }
            } else {
                if is_number {
                    is_number = false;
                    let num: i32 = line[start_pos_number..j]
                        .parse()
                        .expect("Error in parsing number in line");

                    number_hash.insert(Number {
                        value: num,
                        position: (start_pos_number..j)
                            .map(|v| (i as u32, v as u32))
                            .collect(),
                    });
                }

                if c != '.' {
                    symbol_vec.push(Symbol {
                        value: c,
                        position: (i as u32, j as u32),
                    });
                }
            }
        }
        if is_number {
            let num: i32 = line[start_pos_number..line.len()]
                .parse()
                .expect("Error in parsing number in line");

            number_hash.insert(Number {
                value: num,
                position: (start_pos_number..line.len())
                    .map(|v| (i as u32, v as u32))
                    .collect(),
            });
        }
    }
    (number_hash, symbol_vec)
}
