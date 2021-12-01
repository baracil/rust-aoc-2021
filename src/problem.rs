use std::fmt::{Debug};
use std::io;
use std::fs;
use std::fs::read_to_string;

pub enum Part {
    Part1,
    Part2
}
pub struct Problem {
    day:u32,
    input_filename:String
}

impl Problem {

    pub fn new(day:u32) -> Self {
        let input_filename = form_filename(day);
        Self{day,input_filename}
    }

    pub fn read_input(&self) -> io::Result<String> {
        read_to_string(self.input_filename.to_string())
    }

    pub fn read_as_vec_of_line(&self) -> io::Result<Vec<String>> {
        self.read_input().map(to_vec_of_line)
    }

    pub fn read_as_vec_of_u32(&self) -> io::Result<Vec<u32>> {
        self.read_input().map(to_vec_of_int)
    }
}



pub fn to_vec_of_line(content: String) -> Vec<String> {
    content.split("\n")
        .map(|s| s.to_string())
        .collect()
}

pub fn to_vec_of_int(content: String) -> Vec<u32> {
    content.split("\n")
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

fn form_filename(day: u32) -> String {
    return format!("./input/day_{number:0width$}.txt", number = day, width = 2);
}