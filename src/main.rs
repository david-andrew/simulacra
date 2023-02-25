use std::{thread::sleep, time::Duration, iter::zip};
use rand_distr::{Normal, Distribution};
use once_cell::sync::Lazy;
use enum_dispatch::enum_dispatch;
mod screen;
use screen::TTYScreen;
// use screen::TTYScreen;
// use screen::TTYScreen;


/*
[TASKS]
- break into separate files
- decide on time scale for each step (probably make configurable)
- start introducing requirements for humans to survive (air, water, food, temperature, etc): https://pressbooks-dev.oer.hawaii.edu/anatomyandphysiology/chapter/requirements-for-human-life/
    -> along with the corresponding ways for the agent to satisfy those requirements
    1. air. death in 10 minutes. satisfied by environment having air (or eventually outfits that modify the immediate environment for the agent, e.g. a space suit)
        -> tbd on effects when the requirements are not satisfied. e.g. for air, within the 10 minutes before they die, they probably should function less...
    2. water. death in 3 days. satisfied by environment having water sources the agent can go to and drink from
    ...
- start developing AI for agents so that they can recognize their needs and work to satisfy them. GOAP?
*/


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

- character A was injured in battle, and lost their arm
    -> representing limbs on characters
    -> varying degrees of injury

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



/*
[random thoughts]
- path finding algorithms that can precompute all paths (floyd-warshall), but then we can mask part of the graph and quickly find the shortest path on the masked graph
*/


static NORMAL_DIST: Lazy<Normal<f64>> = Lazy::new(||Normal::new(0.0, 1.0).unwrap());
fn randn() -> f64 {
    NORMAL_DIST.sample(&mut rand::thread_rng())
}



// env spaces. e.g. from petting zoo, the dimensionality of the action and observation spaces are known
// trait Space {}
// struct World {
//     actors: Vec<Box<dyn Entity>>,
//     space: Space,
// }

//actors queue actions, and then the world executes them all at once, with conflict resolution.
// enum Action {
//     Move(f64, f64),
//     None,
// }
//action mask?


// trait Act {
//     fn act(&self, world: &World) -> Action;
//     fn resolve(&mut self, action: Action, world: &World);
//     fn draw(&self, display: &mut TTYScreen);
// }



#[enum_dispatch]
trait Act {
    fn act(&self, world: &World) -> Option<Action>; //TODO: mask for actions that are available vs not for the agent
}

struct Person {
    //TODO
}
impl Act for Person {
    fn act(&self, _world: &World) -> Option<Action> {
        //for now, just move in a random direction
        let dx = randn();
        let dy = randn();

        Some(Action::Move(dx, dy))
    }
}
//TODO: other types of agents

#[enum_dispatch(Act)]
enum Actor {
    Person
}

struct Coord {
    x: f64,
    y: f64,
}
struct World {
    actors: Vec<(Actor, Coord)>,
    width: u32,
    height: u32,
}

enum Action
{
    Move(f64, f64),
}

impl World {
    fn new(width: u32, height: u32) -> World {
        World {
            actors: Vec::new(),
            width: width,
            height: height,
        }
    }

    fn add_actor(&mut self, actor: Actor, coord: Coord) {
        self.actors.push((actor, coord));
    }

    fn step(&mut self) {
        // collect attempted actions from all actors
        let mut actions = Vec::new();
        for (actor, coord) in &self.actors {
            actions.push((actor.act(self), coord));
        }

        // resolve conflicts in actions. For now just check that the move action is in bounds
        let mut resolved_actions = Vec::new();
        for ((action, coord), actor_idx) in zip(actions, 0..self.actors.len()) {
            if let Some(action) = action {
                match action {
                    Action::Move(dx, dy) => {
                        let new_x = coord.x + dx;
                        let new_y = coord.y + dy;
                        if new_x >= 1.0 && new_x <= self.width as f64 - 1.0 && new_y >= 1.0 && new_y <= self.height as f64 - 1.0 {
                            resolved_actions.push((action, actor_idx));
                        }
                    },

                    //TODO: other actions
                }
            }
        }

        // execute resolved actions
        for (action, actor_idx) in resolved_actions {
            match action {
                Action::Move(dx, dy) => {
                    let coord = &mut self.actors[actor_idx].1;
                    coord.x += dx;
                    coord.y += dy;
                },
            }
        }
    }

    fn draw(&self, screen: &mut TTYScreen) {
        screen.clear();

        //////// Border around the screen ////////        
        for x in 0..screen.width {
            screen.draw_at(x, 0, '0');
            screen.draw_at(x, screen.height - 1, '0');
        }
        for y in 0..screen.height {
            screen.draw_at(0, y, '0');
            screen.draw_at(screen.width - 1, y, '0');
        }


        for (actor, coord) in &self.actors {
            match actor {
                Actor::Person(_) => {
                    screen.draw_at(coord.x as u32, coord.y as u32, 'X');
                },
            }
        }
        screen.draw();
    }
}

fn stabilize_framerate(frame_start: std::time::Instant, target_frame_duration: Duration) {
    let frame_duration = frame_start.elapsed();
    if frame_duration < target_frame_duration {
        sleep(target_frame_duration - frame_duration);
    }
}

fn main() {
    // constants
    const WIDTH: u32 = 80;
    const HEIGHT: u32 = 24;
    const TARGET_FRAME_RATE: u64 = 60; //frames per second
    const TARGET_FRAME_DURATION: Duration = Duration::from_millis(1000 / TARGET_FRAME_RATE);

    // create the screen and world
    let mut screen = TTYScreen::new(WIDTH, HEIGHT);
    let mut world = World::new(WIDTH, HEIGHT);
    
    // add some actors
    for _ in 0..10 {
        world.add_actor(Actor::Person(Person {}), Coord { x: 40.0, y: 12.0 });
    }

    // game loop
    loop {
        // get the time at the start of the frame
        let frame_start = std::time::Instant::now();

        // update the world
        world.step();

        // draw the world
        screen.clear();
        world.draw(&mut screen);

        // sleep until the target frame rate is reached
        stabilize_framerate(frame_start, TARGET_FRAME_DURATION);
    }
}