# minesweeper-rs
Minesweeper made in rust that can be played in a window or terminal.

![minesweeper](https://user-images.githubusercontent.com/66211581/208497587-1f415cff-2ced-476f-943d-833feb8aacb1.png)

![minewsweeper-terminal](https://user-images.githubusercontent.com/66211581/209406530-f58bb59d-d263-4f1d-a501-a4e348be9569.png)

## Commands
### Window:
- right click: reveals the clicked cell
- left click: put a flag on top of the clicked cell
- R: resets the board
### Terminal:
- enter: reveals cell with the cursor
- spacebar: put a flag on top of the cell with the cursor
- arrow keys: move the cursor
- R: resets the board

## Quick start
Make sure to setup sdl2 correctly if you don't have it already and to put sdl2 dlls in the root folder otherwise it won't work.
If you have never done it you can check sdl2 [crates.io](https://crates.io/crates/sdl2) page.
If you have everything necessary you can just:

To play in a window:
```Console
cargo run -- --window
```

To play in a terminal:
```Console
cargo run -- --terminal
```
