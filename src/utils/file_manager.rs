use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

pub fn load_high_score() -> usize {
    if let Ok(mut file) = File::open("highscore.txt") {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            if let Ok(score) = contents.trim().parse::<usize>() {
                return score;
            }
        }
    }
    0
}

pub fn save_high_score(score: usize) {
    if let Ok(mut file) = OpenOptions::new().write(true).create(true).truncate(true).open("highscore.txt") {
        let _ = write!(file, "{}", score);
    }
} 