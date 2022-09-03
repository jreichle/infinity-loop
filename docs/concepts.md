# Concepts of Infinity Loop

(That's how the game works)

## The game

Infinity loop is a puzzle game built out of a grid of tiles, each with a particular set of connections pointing to orthogonal neighboring tiles. To solve the puzzle, the user must rotate individual tiles to match the connection of all neighboring tiles. The following two images help to demonstrate the visually intuitive ruleset.

**Example of a puzzle:**

![unsolved][unsolvedexample]

**And its solution:**

![solved][solvedexample]

> **_NOTE:_**  A given puzzle may have multiple valid solutions.

**Component Overview:**

* Basic game as [WASM][wasm] Web-UI
* Generating levels with either a
  * unweighted generator, which generates all possible levels with even distribution
  * generator based on [Wave Fuction Collapse][wfc]
* Solving arbitrary puzzle levels with either a
  * solver based on [Constraint Propagation][constraintpropagation]
  * SAT solver
* Manual level editor
* Help during solving by requesting hints

The following section further elaborates on each component. For an overview of the employed architecture and file structure, refer to the [architecture][architecture] file.

## Basic game representation / implementation

[`EnumSet`][enumset] is both a space and performance efficient implementation of a set data structure by relating / associating values with a specific bit in a bit array, usually an unsigned integer. The to possible values of a bit `0` and `1` indicate inclusion in the set. For a type to be eligible to being used in a `EnumSet` requires a bijection between values of the type and the natural numbers. (see the Finite trait)

```rust
struct EnumSet<A>(u64, PhantomData<A>);
```

The fundamental component of interaction in **Infinity Loop** is the `Tile`, which is rotated by the user to solve the puzzle. Conceptually a single tile holds the connection information to its neighbors as a set of directions.

[`Tile`][tile] is a newtype wrapper over an `EnumSet` of directions. Directions correlate to the shape of the tile.

```rust
struct Tile<A>(EnumSet<A>);

enum Square {
    Up,
    Right,
    Down,
    Left
}
```

The rectangular gameboard is modeled by an immutable [`Grid`][grid], which arranges a collection of tiles in a grid structure.

Manipulating elements in the `Grid` is managed by a 2D [`Coordinate`][coordinate] index.

```rust
struct Coordinate<A> {
    row: A,
    column: A
}
```

TestLevel contains some predefined levels for tutorial or test cases. It provides a bunch of gameboards, represented as hardcoded strings, and a deserialization method to create a corresponding Grid.

The progressive change in generated levels is provided by a lazy iterator defined through a [stream unfold][anamorphism] in **levelstream**.

## The UI

Developer: Johannes Reichle

## Level Generator

### Unweighted Generator

Developer: Simon Redl

### The Wave Function Collapse (WFC) Generator

Developer: Alexander Jensen

Wave function collapse is the process by which a system changes from a superposition of states to a discrete state with a clearly defined value of a given measurable quantity by interacting with its environment.

Wave function collapse occurs when a wave function—initially in a superposition of several eigenstates—reduces to a single eigenstate due to interaction with the external world. - [Wiki](https://en.wikipedia.org/wiki/Wave_function_collapse)

## Level Solver

### Constraint Propagation Solver

Developer: Simon Redl

for general idea behind the solver see [Propagators][propagator]

lazily generates all possible solutions for arbitrary levels
strategy: superimpose all possible tiles at each position and successively eliminate them by propagating constraints to neighbors until solved

### SAT-Solver

Developer: Jakob Ritter

1. encode tile configurations and game logic in CNF
2. solve by external SAT-solver
3. decode returned variables

## The Level Editor

Developer: Johannes Moosburger

This part shall provide a editor page, where the user can create his/her own level gameboards. For that, the user can specify a grid (width/height), add tiles of different shapes (use mouse wheel on tile) and rotate (click on a tile) them, to shape a initial level pattern. Furthermore, the board can be tested with both the custom and the SAT-solver, to check, whether the level is a valid one or not. At last, it can be continued with the solving of the puzzle or the gameboard can be serialized, to replay it at another time but in the same browser session.

In short, the editor has following functions:
* Shape level (rotate and change tiles)
* Resize grid 
* Check validity of current level with Constraint-Propagation-Solver
* Check if level is already solved
* Generate level with FastGen
* Generate level with WFC
* Shuffle current grid tile rotations
* Clear grid
* Conintinue with to play the custom grid
* Save grid in local storage
* Load grid in local storage

TODO: können wir auch den schwierigkeitsgrad bestimmen?

[unsolvedexample]: <images/example-level.png>
[solvedexample]: <images/example-level-solution.png>

[propagator]: <https://qfpl.io/share/talks/propagators/slides.pdf>

[wasm]: <https://webassembly.org/>
[wfc]: <https://github.com/mxgmn/WaveFunctionCollapse>
[constraintpropagation]: <https://en.wikipedia.org/wiki/Constraint_satisfaction>
[anamorphism]: <https://en.wikipedia.org/wiki/Anamorphism>

[architecture]: <./architecture.md>

[enumset]: <../game/src/model/enumset.rs>
[coordinate]: <../game/src/model/coordinate.rs>
[tile]: <../game/src/model/tile.rs>
[square]: <../game/src/model/tile.rs>
