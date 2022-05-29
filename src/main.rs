pub mod minesweeper;
pub mod gamedrawer;
pub mod debugging;
use std::io::{self, Write};

fn main(){
    gamedrawer::MinesweeperDrawer::new(9, 9, 10).start_game();
}