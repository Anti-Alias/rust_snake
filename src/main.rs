use std::io::{stdout, Write};
use std::time::Duration;

use crossterm::terminal::{Clear, ClearType};
use crossterm::QueueableCommand;
use snake::{Board, CharPalette, Direction, Food, Vec2, Snake};

const SLEEP_TIME: Duration = Duration::from_millis(500);
const BOARD_WIDTH: u32 = 10;
const BOARD_HEIGHT: u32 = 10;

fn main() {

    // Clears terminal
    let mut stdout = stdout();
    stdout.queue(Clear(ClearType::All)).unwrap();
    stdout.flush().unwrap();

    // Creates snake board
    let snake = Snake::new(Vec2 { x: 0, y: 0 }, Direction::S);
    let food = Food::new(Vec2 { x: 5, y: 5 });
    let mut board = Board::new(BOARD_WIDTH, BOARD_HEIGHT, snake, food);

    // Runs game in loop.
    // Renders to frame buffer, then renders frame buffer to CLI.
    loop {
        board.render_to_stdout(&mut stdout, CharPalette::default());
        stdout.flush().unwrap();
        std::thread::sleep(SLEEP_TIME);
        board.update();
    }
}