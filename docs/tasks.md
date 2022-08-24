
# Tasks

## Project Management

* project architecture overview
* GUI screens and screen sequence sketches

## Core Application

### Graphical User Interface

* custom graphics
* animations

### Controller

* intuitive mouse controls and keyboard shortcuts

### Model

- [X] model for puzzle and functions defining relevant interactions
- [ ] test coverage
    - [X] tile
    - [ ] grid
    - [X] parsing / serializing
- [ ] seperate test levels and parser / serializer into different files
- [ ] establish way of sending gameboard status and updates to View
- [X] implement puzzle generator
- [X] implement puzzle solver

### Level Editor
- [x] Get tiles shown
- [x] Turn cells
- [x] Check with CPS
- [ ] Check with SAT
- [x] Check if level is already solved
- [x] Generate with FastGen
- [x] Generate with WFC
- [x] Change size
- [x] Play custom grid
- [ ] Create user-, not console-messages
- [x] Shuffle current grid tile rotations
- [ ] Adjust editor to level previews


- [ ] Save Grid (optional)
- [ ] Load Grid (optional)

## Further Extensions

* level generator (different strategies possible)
* puzzle solver (different strategies possible)
- [x] import levels from file with parser (parser done, missing GUI)
- [x] export levels to file (persistence) (serializer done, missing GUI)
* game and level statistics
* level editor
* game server to manage level data and level + user statistics
* audio support
