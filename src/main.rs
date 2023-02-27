use std::{time::Duration};

mod screen;
use screen::TTYScreen;

mod utils; use utils::stabilize_framerate;
mod simulacra; use simulacra::{World, Actor, Coord, Person, BuildActor};

/*
[TASKS]
- break into separate files
- decide on time scale for each step (probably make configurable)
    -> time acceleration vs game time between each step
- 
- start introducing requirements for humans to survive (air, water, food, temperature, etc): https://pressbooks-dev.oer.hawaii.edu/anatomyandphysiology/chapter/requirements-for-human-life/
    -> along with the corresponding ways for the agent to satisfy those requirements
    1. air. death in 10 minutes. satisfied by environment having air (or eventually outfits that modify the immediate environment for the agent, e.g. a space suit)
        -> tbd on effects when the requirements are not satisfied. e.g. for air, within the 10 minutes before they die, they probably should function less...
    2. water. death in 3 days. satisfied by environment having water sources the agent can go to and drink from
    ...
- start developing AI for agents so that they can recognize their needs and work to satisfy them. GOAP?
*/






/*
[random thoughts]
- path finding algorithms that can precompute all paths (floyd-warshall), but then we can mask part of the graph and quickly find the shortest path on the masked graph
*/





fn main() {
    // constants
    const WIDTH: u32 = 80;
    const HEIGHT: u32 = 24;
    const TARGET_FRAME_RATE: u64 = 60; //frames per second
    const TARGET_FRAME_DURATION: Duration = Duration::from_micros(1_000_000 / TARGET_FRAME_RATE);
    const TIME_STEP_RATIO: u64 = 60; //how many times faster the simulation is than the real world

    // create the screen and world
    let mut screen = TTYScreen::new(WIDTH, HEIGHT);
    let mut world = World::new(WIDTH, HEIGHT);
    
    // add some actors
    for _ in 0..10 {
        world.add_actor(
            BuildActor::new(Person::default_actor())
                .coord(Coord { x: 40.0, y: 12.0 })
                .has_agency()
        );
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