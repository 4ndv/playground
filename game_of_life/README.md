# Conway's Game of Life

Conway's Game of Life implementation in Rust and Bevy.

This is my very first Bevy project made during free evenings in less than a week, so expect some (lots of) roughness in the code.

## Features

- Panning and zooming
- Adjustable simulation speed
- Step-by-step simulation (press "Enter" when paused)
- Randomizing board state (press "R" when paused)
- Toggling Play/Paused state (press "Space")
- Wrapping around the screen when border is reached

## How state is stored

There is a resource called `Simulation`, which tracks:

- Positions of sprites, also acting as an index for neighbours lookups
- Current population
- Current generation number
- Simulation speed

## Simulation

Because iterating over two dimensional array is kinda slow, and hashlife is too much for my small brain, here's how it currently works:

- Keep an index of all the alive cells, where key is a position of cell, and value is an entity id
- Create three hashsets:
  - List of all affected cells positions (alive cells + their neighbourhood)
  - List of positions where to spawn new cells
  - List of positions where to despawn existing cells
- Iterate the first list, populating other two based on rules
- Spawn and despawn cells
- Repeat
