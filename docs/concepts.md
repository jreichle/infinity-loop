# Concepts of Infinity Loop

**Component Overview:**

* Basic game as [WASM][wasm] Web-UI
* Backend supplying static files
* Generating levels with either a
  * unweighted generator, which generates all possible levels with even distribution
  * generator based on [Wave Function Collapse][wfc]
* Solving arbitrary puzzle levels with either a
  * solver based on [Constraint Propagation][constraintpropagation]
  * SAT solver
* Manual level editor
* Visualization for Wave Function Collapse
* Preview with levels to choose from
* Help during solving by requesting hints
* Using local storage to save current state of single page application

The following section further elaborates on each component. For an overview of the employed architecture and file structure, please refer to the [architecture][architecture] file.

## Basic Game Implementation

[`EnumSet`][enumset] is both a space and performance efficient implementation of a set data structure by associating values with a specific bit in a bit array, usually an unsigned integer. The two possible values of a bit indicate inclusion in the set. For use in `EnumSet<A>`, `A` requires a bijection between values of the type and the natural numbers provided through the [Finite][finite] trait.

```rust
struct EnumSet<A>(u64, PhantomData<A>);
```

The fundamental component of interaction in _Infinity Loop_ is the `Tile`, which is rotated by the user to solve the puzzle. Conceptually a single tile holds the connection information to its neighbors as a set of directions.

[`Tile`][tile] is a newtype wrapper over an `EnumSet` of directions. [Directions][square] correlate to the shape of the tile.

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

The [parser][parser] file contains some predefined levels in string format for test cases as well as deserialization functionality to convert strings into Grids.

The progressive change in generated levels is provided by a lazy iterator defined through a [stream unfold][anamorphism] in [levelstream][levelstream].

## Level Generator

### Unweighted Generator

Assuming the following properties:

1. all possible tiles are available for level generation
2. fixing all orthogonal neighbors uniquely determines the tile

Properties #1 and #2 imply that there always exists a suitable tile for any configuration of neighbors

Generating a valid level now reduces to filling the grid with random tiles in a checkerboard pattern and then infering the blanks based on their neighbors.

### The Wave Function Collapse (WFC) Generator

Wave function collapse is the process by which a system changes from a superposition of states to a discrete state with a clearly defined value of a given measurable quantity by interacting with its environment.

Wave function collapse occurs when a wave function—initially in a superposition of several eigenstates—reduces to a single eigenstate due to interaction with the external world. - [Wiki](https://en.wikipedia.org/wiki/Wave_function_collapse)

## Level Solver

### Constraint Propagation Solver

The level is converted to a grid of possible tiles in superposition surrounded with empty sentinel tiles. The superimposed tiles in the grid are successively reduced by extracting and then propagating common connections to neighbors until solved. A more comprehensive explanation can be found in [Propagationsolver][propagationsolver]. Furthermore all solutions are lazily generated.

The propagation is based on the concept of [Propagators][propagator]

### SAT-Solver

1. encode tile configurations and game logic in CNF
2. solve by external SAT-solver
3. decode returned variables

## Hint Assistance

Algorithm:

1. Generate a trace of the successively solved tiles during solving.
2. Find the first tile in the trace that differs from the equivalent tile in the current level and return the corresponding position.

## Backend

The backend uses the [rocket][rocket] framework for servers.
The purpose of the backend is solely in serving static files and getting the application running in compiling and sending the frontend.
The compilation in [frontend build][build] is facilitated with a rust [build script][build-script].

## The UI

The frontend uses the [yew][yew] framework for building [single-page applications][spa].
While the frontend can be served via the [rocket][rocket] backed server, the frontend can also be run independently.
Yew is heavily inspired by the more popular frontend framework [React][react].
But instead of running with JavaScript, the rust code can be compiled to [WebAssembly][wasm].

The app state is stored into the [local storage][local-storage] which enables the app to retrieve the correct page and selected content therof.
For example, the level is stored when being played.

### Level preview

The level preview provides levels that can be (randomly) chosen to play.
It is possible to load more levels to increase the range of choice.
Additionally a previously saved level can be retrieved.

### Level board

The level board is the part of the application that encompasses the actual playing experience of infinity loop.
This includes the functionality of turning tiles with a mouse click.
A hinting functionality is built in to help a player in case of need.
The solve button, on the other hand, will immediatly complete the level.
Upon completion a new level can be played.

### Wave function collapse visualizer

### The Level Editor

This part shall provide a editor page, where the user can create his/her own level gameboards. For that, the user can specify a grid (width/height), add tiles of different shapes (use mouse wheel on tile) and rotate (click on a tile) them, to shape a initial level pattern.

The editor has the following functions:

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

The editor is based on the _Basic game representatio_. It contains a initial grid, which is replaced or changed by every manipulation during the editing process. To display the grid in HTML notation the board component is used and extended to serve the purpose of the editor. Flags are passed to the component representing cells to enable/disable tile roatation and shape change. Furthermore, various members were add to the `BoardAction`, such as ChangeTileShape, ChangeSize, GenerateFastGen, GenerateWFC, ShuffleTileRotations, ClearGrid. These different actions are handled in the `board_reducer`-file.

[propagator]: <https://qfpl.io/share/talks/propagators/slides.pdf>

[wasm]: <https://webassembly.org/>
[wfc]: <https://github.com/mxgmn/WaveFunctionCollapse>
[constraintpropagation]: <https://en.wikipedia.org/wiki/Constraint_satisfaction>
[anamorphism]: <https://en.wikipedia.org/wiki/Anamorphism>

[architecture]: <./architecture.md>

[enumset]: <../game/src/core/enumset.rs>
[coordinate]: <../game/src/model/coordinate.rs>
[tile]: <../game/src/model/tile.rs>
[square]: <../game/src/model/tile.rs>
[grid]: <../game/src/model/grid.rs>
[finite]: <../game/src/core/finite.rs>
[parser]: <../game/src/model/parser.rs>
[levelstream]: <../game/src/generator/levelstream.rs>
[propagationsolver]: <../game/src/solver/propagationsolver.rs>

[rocket]: <https://rocket.rs/>
[yew]: <https://yew.rs/>
[spa]: <https://en.wikipedia.org/wiki/Single-page_application>
[build]: <../backend/build.rs>
[build-script]: <https://doc.rust-lang.org/cargo/reference/build-scripts.html>
[react]: <https://reactjs.org/>
[local-storage]: <https://en.wikipedia.org/wiki/Web_storage#Local_and_session_storage>
