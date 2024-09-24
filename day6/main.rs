use std::fs;

fn main() {
    let file_content = fs::read_to_string("D:\\Coddingg\\Rust\\AdventOfCode\\day6\\data").expect("Failed to read");

    println!("Advent of code day 6");
    
    let letters: Vec<char> = file_content.chars().collect();
    let mut temp_buffer = String::from("");
    let mut found:bool = false;
    let mut i = 0;
                                             // starting execution for part1 
    while i < letters.len() && !found {
        for j in 0..4 {
            if !temp_buffer.contains(letters[i+j]) {
                temp_buffer += &letters[i+j].to_string();
            }
            else {
                temp_buffer = String::from("");
                break;
            }
            if j == 3 {
                println!("Part1: {}", i + 4);
                found = true;
            }
        }
        i+=1;
    }

    temp_buffer = String::from("");     // Starting execution for part2 
    found = false;
    i = 0;

    while i < letters.len() && !found {
        for j in 0..14 {
            if !temp_buffer.contains(letters[i+j]) {
                temp_buffer += &letters[i+j].to_string();
            }
            else {
                temp_buffer = String::from("");
                break;
            }
            if j == 13 {
                println!("Part2: {}", i + 14);
                found = true
            }
        }
        i += 1;
    }
}