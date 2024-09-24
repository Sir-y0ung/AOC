// Day 3
use std::fs;
use std::collections::HashMap;


fn main() {
    let file_content = fs::read_to_string("D:\\Coddingg\\Rust\\AdventOfCode\\src\\data").expect("Failed to read");
    let mut contents:Vec<[&str;3]> = Vec::new();
    let mut priority_mapping: HashMap<String, u32> = HashMap::new();
    let mut sum = 0;
    let mut count: u32 = 0;
    let mut rucksuck:[&str;3] = ["","",""];

    for line in file_content.lines() {
        if count == 3 {
            contents.push(rucksuck);
            count = 0;
        }  
        if count != 3 {
            // println!("{:?}", line);
            rucksuck[(count % 3) as usize] = line;
        }
        count = count + 1
    }
    contents.push(rucksuck);

    for i in 0..27 {
        let key = (('a' as u8) + i) as char;
        priority_mapping.insert(key.to_string(), (i+1) as u32);
    }

    for i in 0..27 {
        let key = (('A' as u8) + i) as char;
        priority_mapping.insert(key.to_string(), (i+27) as u32);
    }
    
    let letters = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");

    for rucks in contents {
        for letter in letters.chars() {
            for line in rucks {
                if line.contains(letter) {
                    if line == rucks[2] {
                        sum = sum + priority_mapping.get(&String::from(letter)).unwrap();
                        break;
                    }
                    else {
                        continue;
                    }
                }
                else {
                    break;
                }
            }
        }
    }
    println!("Sum is: {}" , sum)
}
