# Contcepts of Infinity Loop
(That's  how the game works)


## The game

Infinity loop is a grid-puzzle, that contains tiles of different shape. These tiles have either one, two, three or four ends (optionally one end to each side of the square grid cell) and can be rotated by the user. The games target is to position all tiles in a way, that no end of a tile is "open" any more. That means, every tile end meets antoher tile end at the grid border (see screenshots).

**Example of a gameboard:**

TODO: add screenshot

**And its solution:**

TODO: add screenshot

One gameboard might have multiple solutions, everyone would leat to a successful end of the level and the beginning of another one.

To extend the basic functionality of this simple game, there are various compontents that make it work and exciting.

**The games components:**
* Basic game implementation
* A Web-UI
* A Wave-Fuction-Collapse Generator (for automated level generation)
* A Custom Solver (for automated level solving)
* A SAT-Solver (for automated level solving)
* A Level Editor (for manual level generation)

In the following, the concepts of every component are explained in its detailes. See the Architectures-Documentation to get an idea, how this conxepts are structured in the project files.


## Basic game representation/implementation

An *EnumSet* is logically a set set that can contain each enum member exactly once. Technically it is implemented as bit-string, where every bit stands for one enum member and toggle its activity. The binary value 0 therefore means the set does not contain the enum member, the binary value 1 means the set does contain the member. 

The smallest unit in this infinity loop game is a *Tile*, which represents one cell in a grid-gameboard. To store, which ends this tiles have and to rotate it easylyt, it extends the *EnumSet*-class for the *Square*-enumeration (members: Up, Right, Down, Left). The tile has all ends that are conatined in the *EnumSet*.
A 90 degree rotation of the tile corresponds to a circular bit shift by one. (left shift = counter clockwise, right shift = clockwise)

One gameboard is then represented by an object of the *Grid*-class, which contains a number of tiles. During the game, these tile are manipulated. The *Grid*-class also provides a function to check, if the puzzle is solved or not.

*TestLevel* contains some predefined levels for tutorial or test cases. It provides a bunch of gameboards, represented as hardcoded strings, and a deserialization method to create a corresponding Grid.

## The UI
Developer: Johannes Reichle


## The Wave Function Collapse Generator
Developer: Alexander Jensen

## Level Solver
### Custom Solver
Developer: Simon Redl

general solving strategy: from most to least restricted

1. 0-tiles and 4-tiles are trivially solved and can be excluded
2. 3-tiles and I-2-tiles on the edges, L-2-tiles in the corners have only a single valid configuration
3. apply backtracking / determine tiles with least valid configurations / (some other heuristic)

### SAT-Solver
Developer: Jakob Ritter

1. encode tile configurations and game logic in CNF
2. solve by external SAT-solver
3. decode returned variables


## The Level Editor
Developer: Johannes Moosburger

This part shall provide a editor page, where the user can create his/her own level gameboards. For that, the user can specify a grid (width/height), add tiles of different shapes and rotate them, to shape a initial level pattern. Furthermore, the board can be tested with both the custom and the SAT-solver, to check, whether the level is a valid one or not. At last, it can be continued with the solving of the puzzle.

Optional: The gameboard can be serialized, to replay it in another session.

TODO: können wir auch den schwierigkeitsgrad bestimmen?