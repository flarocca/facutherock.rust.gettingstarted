use std::io::{stdin, Stdin};
use crate::models::board::Cell;

pub trait Inputter {
    fn get_cell(&self) -> Cell;
}

pub struct StdinInputter {
    stdin: Stdin
}

impl StdinInputter {
    pub fn new () -> Self {
        StdinInputter{
            stdin: stdin()
        }
    }
}

impl Inputter for StdinInputter {

    fn get_cell(&self) -> Cell {

        let mut line = String::new();

        println!("Enter the cell you want to take");
    
        println!("X: ");
    
        line.clear();
        self.stdin.read_line(&mut line).unwrap();
    
        let x = line.trim().to_string();
        line = x.clone();
    
        if x == "exit" {   
            return Cell::empty();
        }
    
        println!("Y: ");
    
        line.clear();
        self.stdin.read_line(&mut line).unwrap();
        let y = line.trim().to_string();
    
        if y == "exit" {
            return Cell::empty();
        }
    
        Cell {
            x: x.parse::<usize>().unwrap(),
            y: y.parse::<usize>().unwrap()
        }
    }
}