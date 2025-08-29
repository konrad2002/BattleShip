# BattleShip
A rust server for the game battle ship and example react client.

## Server Rest API

The following endpoints are defined and return/take the give data objects:

### `GET /boards/{side}`

- returns both boards modified for the viewing side
- PARAMS:
  - `side` is either 1 or 2
- RETURN (example):
```json
{
  "id": 2341,
  "boards": {
    "size": 4,
    "player": [
      0, 1, 0, 0, 0,
      3, 3, 2, 0, 4,
      0, 0, 0, 0, 4,
      0, 1, 0, 0, 1,
      1, 2, 2, 2, 0
    ],
    "enemy": [
      1, 1, 0, 0, 0,
      0, 1, 1, 3, 0,
      4, 0, 0, 0, 0,
      4, 0, 4, 0, 1,
      4, 0, 4, 1, 1
    ]
  },
  "winner": 0
}
```

This example represents the following game state:

**Player itself (1st) and enemy (2nd)**

<img src="docs/image/ship1.png" width="140px" style="display: inline-block">
<img src="docs/image/ship2.png" width="140px" style="display: inline-block">

The numbers from the list have the following meanings:

- in boards:
  - 0: nothing
  - 1: shot but free
  - 2: ship (no hit)
  - 3: ship (hit)
  - 4: ship (hit and destroyed)
- winner:
  - 0: nobody
  - 1: player 1
  - 2: player 2

Therefore, the array `enemy` will never contain number 2. The matrix representation can be derived from the `size`. The game is always a square.

### `POST /board/{side}/init`

- sets the initial board layout at the beginning for a specified side
- PARAMS:
  - `side` is either 1 or 2
- REQUEST BODY:
```json
{
  "board": [
    0, 0, 0, 0, 0,
    2, 2, 2, 0, 2,
    0, 0, 0, 0, 2,
    0, 0, 0, 0, 0,
    0, 2, 2, 2, 0
  ]
}
```
- RETURN:
  - board for this side

### `POST /board/{side}/shoot/{pos}`

- shoots a 
- PARAMS:
  - `side` is either 1 or 2
  - `pos` is the position of the field starting to count from top left with 0
- RETURN:
  - like `GET /boards/{side}`