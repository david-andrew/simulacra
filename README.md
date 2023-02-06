# Simulacra
Simulations of human behavior. The goal is to create efficient, high fidelity simulations of humans in various environments and replicate known behavioral phenomena. This work aims to provide a solid base system for managing NPC actions in video games.

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
## Display Features
- [x] 2D ascii terminal display