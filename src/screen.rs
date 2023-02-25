
use std::io::{self, Write};

// screen for now will be drawn in a terminal window with ascii characters
pub struct TTYScreen {
    pub width: u32,
    pub height: u32,
    data: Vec<char>,
}

impl TTYScreen {
    pub fn new(width: u32, height: u32) -> TTYScreen {
        
        //create the data vector
        let mut data = Vec::new();
        for _ in 0..width * height {
            data.push(' ');
        }
        
        //predraw the space for the screen, then return to the top left
        for _ in 0..height-1 {
            println!("")
        }
        print!("\x1b[0;0H");
        
        //return a new TTYScreen
        TTYScreen {
            width: width,
            height: height,
            data: data,
        }
    }

    pub fn draw_at(&mut self, x: u32, y: u32, c: char) {
        self.data[(y * self.width + x) as usize] = c;
    }

    pub fn clear(&mut self) {
        for i in 0..self.width * self.height {
            self.data[i as usize] = ' ';
        }
    }

    pub fn draw(&self) {
        //first move the cursor to the top left
        print!("\x1b[0;0H");

        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.data[(y * self.width + x) as usize]);
            }
            if y < self.height - 1 {
                println!("");
            }
        }
        //flush the output
        io::stdout().flush().unwrap();

    }
}