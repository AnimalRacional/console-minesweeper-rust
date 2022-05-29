use crate::minesweeper::{MinesweeperGame, Squares, State, Point};
use crate::debugging;
use std::io::{self, Write};

pub struct MinesweeperDrawer {
    game: MinesweeperGame
}


// Graphics
impl MinesweeperDrawer {
    pub fn new(xsize: i32, ysize: i32, bomb_amount: i32) -> MinesweeperDrawer{
        MinesweeperDrawer{
            game: MinesweeperGame::new(xsize, ysize, bomb_amount)
        }
    }

    fn get_num() -> i32 {
        loop {
            let mut buf = String::new();
            if let Ok(_) = io::stdin().read_line(&mut buf){
                match buf.trim().parse() {
                    Ok(a) => { return a; },
                    Err(_) => { println!("Please insert a valid number!"); continue; }
                };
            }
        }
    }

    fn get_location() -> Point {
        println!("X coordinate:");
        let x = MinesweeperDrawer::get_num();
        println!("Y coordinate:");
        let y = MinesweeperDrawer::get_num();
        Point::new(x,y)
    }

    fn do_action(&mut self){
        loop{
            println!("Choose an action: reveal or flag");
            let mut buf = String::new();
            if let Ok(_) = io::stdin().read_line(&mut buf){
                buf = buf.trim().to_string();
                if buf.eq_ignore_ascii_case("reveal") {
                    let loc = MinesweeperDrawer::get_location();
                    self.game.reveal(loc.x, loc.y);
                }
                else if buf.eq_ignore_ascii_case("flag") {
                    let loc = MinesweeperDrawer::get_location();
                    self.game.flag(loc.x, loc.y);
                }
                else{
                    println!("Invalid action!");
                    continue;
                }
                break;
            }
        }
    }

    
    pub fn start_game(&mut self) {
        while self.game.game_state == State::Ongoing {
            self.draw_board();
            self.do_action();
        }
        if self.game.game_state == State::Lost {
            self.draw_board();
            println!("You lose!");
        }
        else if self.game.game_state == State::Won {
            self.draw_board();
            println!("You win!");
        }
    }

    fn draw_board(&self){
        for x in 0..self.game.board_size.x {
            for y in 0..self.game.board_size.y{
                let id = self.game.calculate_index_by_coords(x, y);
                print!("{}", match self.game.board[id as usize]{
                    Squares::ClosedBomb => { "?".to_string() },
                    Squares::ClosedSafe => { "?".to_string() },
                    Squares::FlaggedBomb => { "X".to_string() },
                    Squares::FlaggedSafe => { "X".to_string() },
                    Squares::OpenSafe => { self.game.calculate_neighbours(x, y).to_string() }
                });
            }
            println!();
        }
    }
}