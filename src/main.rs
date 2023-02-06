use rand_distr::{Normal, Distribution, num_traits::clamp};
use std::{io::{self, Write}, thread::sleep, time::Duration};



// TODO:
// - move screen into separate file
// - convert things like randn and sleep into closures/functions that are easier to call
// - types of economies to simulate:
//    -> start off with small communal villages
//    -> simulate at the individual level
//       - needs like food, water, shelter, etc
//       - wants like entertainment, education, etc
//       - skills like hunting, farming, etc ---> this may turn into an efficiency score
//       - voting? other forms of social interaction?
//       - efficiency score + goal oriented action planning may be used to determine what the agent does
//       - agent health/injuries/etc. 
//          -> for injuries, maybe have a human armature that models the workspace the agent can reach, and update that based on injuries

// recreate actor class from python version
/*
class Actor(ABC):
    @abstractmethod
    def step(self): ...
    def render(self, screen:Screen): ... #TBD what how this works...

class Person(Actor):
    def __init__(self):
        self.x = 10
        self.y = 10

    def step(self):
        #for now, just move in a random direction
        dx = np.random.randn()
        dy = np.random.randn()

        self.x += dx
        self.y += dy
        
        #clamp to area
        self.x = np.clip(self.x, 0, 80)
        self.y = np.clip(self.y, 0, 80)

    def render(self, screen:Screen):
        screen.draw_at("@", int(self.x), int(self.y))
*/



// screen for now will be drawn in a terminal window with ascii characters
struct TTYScreen {
    width: u32,
    height: u32,
    data: Vec<char>,
}

impl TTYScreen {
    fn new(width: u32, height: u32) -> TTYScreen {
        
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

    fn draw_at(&mut self, x: u32, y: u32, c: char) {
        self.data[(y * self.width + x) as usize] = c;
    }

    fn clear(&mut self) {
        for i in 0..self.width * self.height {
            self.data[i as usize] = ' ';
        }
    }

    fn draw(&self) {
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


fn main()
{
    const WIDTH: u32 = 80;
    const HEIGHT: u32 = 24;
    const SLEEP_DURATION: Duration = Duration::from_millis(10); 

    let mut screen = TTYScreen::new(WIDTH, HEIGHT);
    
    //coordinates of X character
    let mut x:f64 = WIDTH as f64 / 2.0;
    let mut y:f64 = HEIGHT as f64 / 2.0;
    println!("x: {}, y: {}", x, y);

    //random number generator
    let normal = Normal::new(0.0, 1.0).unwrap();

    //game loop
    loop {
        //////// Clear the screen ////////
        screen.clear();
        

        //////// World updates go here ////////

        //move the x by a random amount
        x += normal.sample(&mut rand::thread_rng());
        y += normal.sample(&mut rand::thread_rng());
        
        //clamp x and y into screen bounds
        x = clamp(x, 1.0, WIDTH as f64 - 2.0);
        y = clamp(y, 1.0, HEIGHT as f64 - 2.0);
        
        //draw the x at the current position (converted to integer)
        screen.draw_at(x as u32, y as u32, 'X');
        

        
        //////// Border around the screen ////////        
        for x in 0..screen.width {
            screen.draw_at(x, 0, '0');
            screen.draw_at(x, screen.height - 1, '0');
        }
        for y in 0..screen.height {
            screen.draw_at(0, y, '0');
            screen.draw_at(screen.width - 1, y, '0');
        }



        //////// draw the screen, and sleep for a bit ////////
        screen.draw();
        sleep(SLEEP_DURATION);
    }
        
}
