// Day 2
use std::fs;
static mut PLAYER2_SCORE:i32 = 0;
const WIN:i32  =  6;
const DRAW:i32 = 3;

fn check_result(moves:&str) {
    let choices:Vec<&str> = moves.split_whitespace().collect();
    
    // win situation
    unsafe {
        match choices[1]  {
            "X" => {
                if choices[0] == "A" {
                    PLAYER2_SCORE = PLAYER2_SCORE + 3
                }
                else if choices[0] == "B" {
                    PLAYER2_SCORE = PLAYER2_SCORE + 1
                }
                else {
                    PLAYER2_SCORE = PLAYER2_SCORE + 2
                }
            }
            "Y" => {
                if choices[0] == "A" {
                    PLAYER2_SCORE = PLAYER2_SCORE + DRAW + 1
                }
                else if choices[0] == "B" {
                    PLAYER2_SCORE = PLAYER2_SCORE + DRAW + 2
                }
                else {
                    PLAYER2_SCORE = PLAYER2_SCORE + DRAW + 3
                }
            }
            "Z" => {
                if choices[0] == "A" {
                    PLAYER2_SCORE = PLAYER2_SCORE + WIN + 2
                }
                else if choices[0] == "B" {
                    PLAYER2_SCORE = PLAYER2_SCORE + WIN + 3
                }
                else {
                    PLAYER2_SCORE = PLAYER2_SCORE + WIN + 1
                }
            }
            _ => println!("Not a valid move."),
        }
    }
}

fn main() {
    let file_content = fs::read_to_string("data").expect("Failed to read");
    let mut contents:Vec<&str> = Vec::new();

    
    for line in file_content.lines() {
        contents.push(line);
    }

    for line in contents {
        check_result(&line);
    }

    unsafe {
        println!("Player2 score: {}", PLAYER2_SCORE);
    } 
}
