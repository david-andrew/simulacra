use std::iter::zip;
use enum_dispatch::enum_dispatch;

use crate::utils::randn;
use crate::screen::TTYScreen;



/*
person system:
- lungs:
    - take in air
    - extract oxygen
    - replace with carbon dioxide
    - oxygenate blood with extracted oxygen
    - require energy
- blood:
    - transports oxygen and nutrients to cells
    - transports carbon dioxide and waste away from cells
- heart:
    - link blood to all other organs
    - requires energy
- brain:
    - 
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



[Agent planning]
- agents will use some form of planning to decide what to do
    -> probably GOAP, maybe mixed with some form of neural net (maybe evolution trained)
-> characters can break up GOAP planning over multiple frames! since thinking takes time, they just sit there and think. fixes performance bottleneck too!






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

pub struct Person {
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
pub enum Actor {
    Person
}

pub struct Coord {
    pub x: f64,
    pub y: f64,
}
pub struct World {
    actors: Vec<(Actor, Coord)>,
    width: u32,
    height: u32,
}

enum Action
{
    Move(f64, f64),
}

impl World {
    pub fn new(width: u32, height: u32) -> World {
        World {
            actors: Vec::new(),
            width: width,
            height: height,
        }
    }

    pub fn add_actor(&mut self, actor: Actor, coord: Coord) {
        self.actors.push((actor, coord));
    }

    pub fn step(&mut self) {
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

    pub fn draw(&self, screen: &mut TTYScreen) {
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


