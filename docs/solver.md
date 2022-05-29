
# Solvers for Infinity Loop

## Custom Solver

general solving strategy: from most to least restricted

1. 0-tiles and 4-tiles are trivially solved and can be excluded
2. 3-tiles and I-2-tiles on the edges, L-2-tiles in the corners have only a single valid configuration
3. apply backtracking / determine tiles with least valid configurations / (some other heuristic)

## SAT-Solver

1. encode tile configurations and game logic in CNF
2. solve by external SAT-solver
3. decode returned variables