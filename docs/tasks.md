
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

- [x] model for puzzle and functions defining relevant interactions
- [ ] test coverage
    - [x] tile
    - [ ] grid
    - [ ] parsing / serializing
- [ ] seperate test levels and parser / serializer into different files
- [ ] establish way of sending gameboard status and updates to View
- [ ] implement puzzle generator
- [ ] implement puzzle solver

## Further Extensions

* level generator (different strategies possible)
* puzzle solver (different strategies possible)
- [x] import levels from file with parser (parser done, missing GUI)
- [x] export levels to file (persistence) (serializer done, missing GUI)
* game and level statistics
* level editor
* game server to manage level data and level + user statistics
* audio support
