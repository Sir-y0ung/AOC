// Day 2
use std::fs;
use std::collections::HashMap;


static mut PLAYER1_SCORE:i32 = 0;
static mut PLAYER2_SCORE:i32 = 0;
const WIN:i32  =  6;
const DRAW:i32 = 3;

fn check_result(moves:&str) {
    let choices:Vec<&str> = moves.split_whitespace().collect();
    let mut game_codes = HashMap::new();
    game_codes.insert("A", 1);
    game_codes.insert("B", 2);
    game_codes.insert("C", 3);
    
    game_codes.insert("X", 1);
    game_codes.insert("Y", 2);
    game_codes.insert("Z", 3);

    //Draw situation
    if game_codes.get(choices[0]) == game_codes.get(choices[1]) {
        unsafe {
            PLAYER1_SCORE = PLAYER1_SCORE + DRAW + game_codes.get(choices[0]).unwrap();
            PLAYER2_SCORE = PLAYER2_SCORE + DRAW + game_codes.get(choices[1]).unwrap();
        }
        return;
    }
    
    // win situation
    unsafe {
        match choices[0]  {
            "A" => {
                if choices[1] == "Y" {
                    PLAYER1_SCORE = PLAYER1_SCORE + game_codes.get(choices[0]).unwrap();
                    PLAYER2_SCORE = PLAYER2_SCORE + WIN + game_codes.get(choices[1]).unwrap();
                }
                else if choices[1] == "Z" {
                    PLAYER1_SCORE = PLAYER1_SCORE + WIN + game_codes.get(choices[0]).unwrap();
                    PLAYER2_SCORE = PLAYER2_SCORE + game_codes.get(choices[1]).unwrap();
                }
            }
            "B" => {
                if choices[1] == "Z" {
                    PLAYER1_SCORE = PLAYER1_SCORE + game_codes.get(choices[0]).unwrap();
                    PLAYER2_SCORE = PLAYER2_SCORE + WIN + game_codes.get(choices[1]).unwrap();
                }
                else if choices[1] == "X" {
                    PLAYER1_SCORE = PLAYER1_SCORE + WIN + game_codes.get(choices[0]).unwrap();
                    PLAYER2_SCORE = PLAYER2_SCORE + game_codes.get(choices[1]).unwrap();
                }
            }
            "C" => {
                if choices[1] == "X" {
                    PLAYER1_SCORE = PLAYER1_SCORE + game_codes.get(choices[0]).unwrap();
                    PLAYER2_SCORE = PLAYER2_SCORE + WIN + game_codes.get(choices[1]).unwrap();
                }
                else if choices[1] == "Y" {
                    PLAYER1_SCORE = PLAYER1_SCORE + WIN + game_codes.get(choices[0]).unwrap();
                    PLAYER2_SCORE = PLAYER2_SCORE + game_codes.get(choices[1]).unwrap();
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
        println!("Player1 score: {}", PLAYER1_SCORE);
        println!("Player2 score: {}", PLAYER2_SCORE);
    } 
}
