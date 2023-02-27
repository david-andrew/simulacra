use std::{collections::{HashMap, HashSet}};
use enum_dispatch::enum_dispatch;

use crate::utils::randn;
use crate::screen::TTYScreen;



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
    fn act(&self, world: &World) -> Action; //TODO: mask for actions that are available vs not for the agent
}




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
pub struct Person {
    //TODO
}
impl Act for Person {
    fn act(&self, _world: &World) -> Action {
        //for now, just move in a random direction
        let dx = randn();
        let dy = randn();

        Action::Move(Move{dx, dy})
    }
    
}
impl Person {
    pub fn default_actor() -> Actor {
        Actor::Person(Person{})
    }
}
//TODO: other types of agents

pub struct Lungs {
    //consumes: surrounding atmosphere, energy, un-oxygenated blood
    //produces: oxygenated blood
    // health: f64,
    // parent: &Actor,
}

impl Act for Lungs {
    fn act(&self, _world: &World) -> Action {
        Action::None
    }
}

struct Blood {
    //bus for transporting oxygen, carbon dioxide, nutrients, waste
}
impl Act for Blood {
    fn act(&self, _world: &World) -> Action {
        Action::None
    }
}
struct Heart {}


struct Brain {
    //allows an agent to make plans. thinking speed + max size of plans is based on the free size of the brain
    //consumes: energy + oxygen (from blood)
    //produces: GOAP actions
    //links: Vec<Actor>,
    //free_size: f64 //amount of brain that is not needed for controlling body functions

    //brain could be mirrored for left vs right handedness
    //also there could be other odd brain layouts, e.g. for people with autism (smart but not social)
    //  re: brain layouts, probably have a more explicit specification that is allocation based, that would play into personality traits

    // potentially split brain into a higher and lower brain? e.g. one for pure instinct based survival and then more advanced for planning
}
struct Stomach {
    // convert food into energy
}
struct Eyes {}






pub struct BuildActor {
    actor: Actor,
    coord: Option<Coord>,
    has_agency: Option<()>,
    //TODO: other parameters set when adding an actor to the world
}
impl BuildActor {
    pub fn new(actor: Actor) -> Self {
        Self {
            actor,
            coord: None,
            has_agency: None,
        }
    }
    pub fn coord(mut self, coord: Coord) -> Self {
        self.coord = Some(coord);
        self
    }
    pub fn has_agency(mut self) -> Self {
        self.has_agency = Some(());
        self
    }
}


#[enum_dispatch(Act)]
pub enum Actor {
    Person,
    Lungs,
    Blood,
    //Heart,
    //Brain,
    //Eyes,
    //Stomach,
}


pub struct Coord {
    pub x: f64,
    pub y: f64,
}
type ID = u32;
pub struct World {
    cur_id: ID,
    // live_ids: HashSet<ID>,
    agency: HashSet<ID>,               //used for several things. for now, no agency means other agents can trade without needing mirrored trades
    coordinates: HashMap<ID, Coord>,
    actors: Vec<(ID, Actor)>,
    width: u32,
    height: u32,
    
    
    // [Connection Graphs]
    //  - possessions. e.g. blood possesses oxygen, person possesses jacket, etc. but perhaps this should be split into active and passive, e.g. a person couldn't trade their lungs with someone, but they could trade their jacket, even though they possess both
    //  - touching... tbd. trying to handle how a person would breath air in from the atmosphere

    //live_ids: 
    //coordmap
    //inside of graph (e.g. organs inside a person). These objects share coordinates with the parent
    //    -> outside of graph (e.g. jacket on a person). These objects share coordinates with the parent
    //    -> possibly have just a single "subordinate" graph for objects that share coordinates with the parent
    //physically connected graph (e.g. lungs connected to heart)
    //other graphs. e.g. relationships, etc.
}

enum Action
{
    None,
    Move(Move),
    Trade(Trade), // --> succeeds only if a mirrored trade is being made by the target agent
}
struct Move {
    dx: f64,
    dy: f64,
}
struct Trade {
    target_id: ID,
    give_id: ID,
    receive_id: ID,
}

impl World {
    pub fn new(width: u32, height: u32) -> World {
        World {
            cur_id: 0,
            actors: Vec::new(),
            // alive: HashSet::new(),
            agency: HashSet::new(),
            coordinates: HashMap::new(),
            width: width,
            height: height,
        }
    }

    // set up a builder pattern for adding actors to the world.
    pub fn add_actor(&mut self, new_actor: BuildActor) {
        let id = self.cur_id;
        self.actors.push((id, new_actor.actor));
        
        if let Some(coord) = new_actor.coord {
            self.coordinates.insert(id, coord);
        }
        
        if let Some(_) = new_actor.has_agency {
            self.agency.insert(id);
        }
        
        self.cur_id += 1;
    }

    pub fn step(&mut self) {
        // collect attempted actions from all actors
        let actions: HashMap<&ID, Action> = self.actors
            .iter()
            .map(|(id, actor)| (id, actor.act(self)))
            .collect();
            
        
        // resolve conflicts in actions. For now just check that the move action is in bounds
        let mut resolved_actions: Vec<(&ID, &Action)> = Vec::new();
        for (id, action) in &actions {
            match action {
                Action::Move(Move{dx, dy}) => {
                    let coord = self.coordinates.get(id);
                    if let Some(coord) = coord {
                        let new_x = coord.x + dx;
                        let new_y = coord.y + dy;
                        if new_x >= 1.0 && new_x <= self.width as f64 - 1.0 && new_y >= 1.0 && new_y <= self.height as f64 - 1.0 {
                            resolved_actions.push((id, action));
                        }
                    }
                },
                Action::Trade(a) => {
                    //check if the target agent also has a mirrored trade action
                    if let Some(Action::Trade(b)) = actions.get(&a.target_id) {
                        if a.give_id == b.receive_id && a.receive_id == b.give_id && b.target_id == **id {
                            resolved_actions.push((id, action));
                        }
                    }
                }
                
                //TODO: other actions
                Action::None => {}
            }
        }

        // execute resolved actions
        for (id, action) in resolved_actions {
            match action {
                Action::Move(Move{dx, dy}) => {
                    // let coord = //&mut self.actors[id].1;
                    let coord = self.coordinates.get_mut(id);
                    if let Some(coord) = coord {
                        coord.x += dx;
                        coord.y += dy;
                    }
                },
                Action::Trade(Trade{target_id, give_id, receive_id}) => {
                    //TODO
                }

                Action::None => {}
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


        for (id, actor) in &self.actors {
            match actor {
                Actor::Person(_) => {
                    let coord = self.coordinates.get(id);
                    if let Some(coord) = coord {
                        screen.draw_at(coord.x as u32, coord.y as u32, 'X');
                    }
                },
                _ => {},
            }
        }
        screen.draw();
    }
}


