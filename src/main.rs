use rand_distr::{Normal, Distribution, num_traits::clamp};
use std::{io::{self, Write}, thread::sleep, time::Duration};


/*
User stories:
- character A is hungry, so they pick a fruit from a tree and eat it
    -> plants that grow and produce fruit

- character A is tired, so they go to shelter and sleep
    -> shelter owned by or provided to or found by character A

- character A has harvested some wheat, and goes to a market to trade for other goods
    -> plants grow and produce wheat
    -> other characters that have other goods
    -> ability for characters to negotiate and trade based on their needs and wants

- character A is tending sheep, and must protect them from wolves
    -> sheep are animal agents
    -> wolves are animal agents

<stealing>

<delegating tasks>

<hierarchical and non-hierarchical collaboration structures>

<knowledge chains/information propagation + mutation>

<lying, deception, information withholding>

<trust>!!!!!! super big one . The evolution of trust: https://ncase.me/trust/

<coercion>

<cost benefit analysis. opportunity cost>


- kingdom A wants to trade its wheat for kingdom B's meat
    -> kingdoms need to direct individuals to collect/pool the resources
    -> kingdoms then need to direct individuals to transport the resources to the other kingdom
    -> kingdoms then need to verify that the trade occurred successfully

- kingdom A decides to invade kingdom B to prevent them from becoming too powerful
    -> some way for leaders to measure the power/trajectory of other kingdoms
    -> cost benefit analysis
    -> TBD


breakdown of the interface:
struct Environment:
    - vector<agents>: all actors in the environment
    - world: the world the agents are in, including non-agent entities

struct Agent:
    - type (human, animal, plant, etc)
    - position
    - state. e.g. for humans https://en.wikipedia.org/wiki/Maslow%27s_hierarchy_of_needs
trait Agent
    - act()

World:
    - map/environments
    - weather-> is this maybe an agent?
    - resources (non-agent)
    - time


- Screen:
    - view of the world
    - controls for looking around


- User controlled agents:
   - mainly for debugging

*/







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



/*
Replicate Env/Actor setup from python/pytorch petting zoo
env.reset()
for agent in env.agent_iter():
    observation, reward, termination, truncation, info = env.last()
    action = None if termination or truncation else env.action_space(agent).sample()  # this is where you would insert your policy
    env.step(action)

Though actually it would probably be more like this:
env.reset()
for agent in env.agents:
    prev_reward = env.prev_reward(agent)
    observation = env.observation(agent)
    action = env.action_space(agent).sample()  # or some policy
    env.step(action)
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


// trait Space<T> {
//     fn is_in_bounds(&self, coords: T) -> bool;
// }


// // simple 2D plane world
// struct PlaneSpace {
//     width: u32,
//     height: u32,
// }
// impl Space<(f64, f64)> for PlaneSpace {
//     fn is_in_bounds(&self, coords: (f64, f64)) -> bool {
//         coords.0 >= 0.0 && coords.0 < self.width as f64 && coords.1 >= 0.0 && coords.1 < self.height as f64
//     }
// }

// struct Env {
//     actors: Vec<Box<dyn Actor>>,
//     space: dyn Space
// }
// trait Env {}
// trait Actor {}
// trait Space {}


// struct World<T> {
//     actors: Vec<Box<dyn Entity>>,
//     space: Space<T>,

// }


// trait Entity {
//     fn draw(&self, screen: &mut TTYScreen);
//     fn step(&mut self, world: &World);
// }


// struct Person {
//     x: f64,
//     y: f64,
// }

// impl Entity for Person {
//     fn step(&mut self, world: &World) {
//         //for now, just move in a random direction
//         let normal = Normal::new(0.0, 1.0).unwrap();
//         let dx = normal.sample(&mut rand::thread_rng());
//         let dy = normal.sample(&mut rand::thread_rng());

//         self.x += dx;
//         self.y += dy;
        
//         //clamp to area
//         self.x = clamp(self.x, 0.0, 80.0);
//         self.y = clamp(self.y, 0.0, 80.0);
//     }

//     fn draw(&self, screen: &mut TTYScreen) {
//         screen.draw_at(self.x as u32, self.y as u32, '@');
//     }
// }




trait Act<World> {
    fn step(&mut self, world: &World);
}

struct PlaneWorld {
    actors: Vec<Box<dyn Act<PlaneWorld>>>,
    width: u32,
    height: u32,
}




fn main()
{
    const WIDTH: u32 = 80;
    const HEIGHT: u32 = 24;
    const TARGET_FRAME_RATE: u64 = 60; //frames per second
    const TARGET_FRAME_DURATION: Duration = Duration::from_millis(1000 / TARGET_FRAME_RATE);


    let mut screen = TTYScreen::new(WIDTH, HEIGHT);
    
    //coordinates of X character
    let mut x:f64 = WIDTH as f64 / 2.0;
    let mut y:f64 = HEIGHT as f64 / 2.0;
    println!("x: {}, y: {}", x, y);

    //random number generator
    let normal = Normal::new(0.0, 1.0).unwrap();

    //game loop
    loop {
        let frame_start = std::time::Instant::now();
        
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



        //////// draw the screen, and sleep for remainder of frame ////////
        screen.draw();
        let current_frame_duration = std::time::Instant::now() - frame_start;
        let sleep_duration = TARGET_FRAME_DURATION - current_frame_duration;
        if sleep_duration > Duration::from_millis(0) {
            sleep(sleep_duration);
        }

    }
        
}
