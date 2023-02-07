# Simulacra
Simulations of human behavior. The goal is to create efficient, high fidelity simulations of humans in various environments and replicate known behavioral phenomena. This work aims to provide a solid base system for managing NPC actions in video games.

My other goal for this project is to build experience in rust.

## Prerequisites
- Rust/Cargo

## Running the Demo

```bash
$ cargo run
```

## Simulation Features
- [ ] world
    - [x] real-vector position in finite a 2D plane
    - [ ] resources
    - [ ] terrain
    - [ ] elements/effects
    - [ ] TBD 
- [ ] actors
    - [ ] hunger
    - [ ] thirst
    - [ ] energy/fatigue
    - [ ] injuries
        - [ ] human armature that simulates reachable workspace, and is effected by injuries
    - [ ] age/growth
    - [ ] planning
        - [ ] Goal Oriented Action Planning (GOAP). What about planning with uncertainty? -> expected outcomes?
    - [ ] collaboration
    - [ ] communication
    - [ ] tools/fabrication
- [ ] Information/Knowledge flow
    - [ ] all information consists of logical propositions. some examples:
        - there is a town 1 mile north of here
        - character A was alive yesterday
        - character B is a friend of character A
        - etc.
    - [ ] all propositions are stored in a central location. Actors can have pointers to particular pieces of knowledge
    - [ ] actors create new propositions, meaning new knowledge is created. A proposition is delete when no actors have a reference to it.
    - [ ] actors can transfer knowledge to other actors (depending on things like trust, deception, plausibility, confirmation bias match, etc.)
    - [ ] GOAP makes use of knowledge. should have probabilistic approach for when knowledge conflicts?
## Display Features
- [x] 2D ascii terminal display