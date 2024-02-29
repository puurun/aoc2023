

pub mod problems;

fn main() {
    println!("---- Day 1: Trebuchet ----");
    println!("Part 1: Missing :(, Didn't save it");
    print!("Part 2: ");
    problems::day1_trebuchet::solve();


    println!();
    println!("---- Day 2: Cube Conundrum ----");
    print!("Part 1: ");
    problems::day2_cube_conundrum::solve();
    print!("Part 2: ");
    problems::day2_cube_conundrum::solve2();


    println!();
    println!("---- Day 3: Gear Ratios ----");
    print!("Part 1: ");
    problems::day3_gear_ratios::solve();
    print!("Part 2: ");
    problems::day3_gear_ratios::solve2();

    println!();
    println!("---- Day 4: Scratchcards ----");
    print!("Part 1: ");
    problems::day4_scratchcards::solve();
}
