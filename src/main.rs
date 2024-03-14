use snake::{Board, CharPalette, Direction, Food, Position, Snake};

const BOARD_WIDTH: u32 = 10;
const BOARD_HEIGHT: u32 = 10;
const SEPARATION_CHAR: char = ' ';

fn main() {
    
    // Creates snake board
    let snake = Snake::new(Position { x: 0, y: 0 }, Direction::E);
    let food = Food::new(Position { x: 5, y: 5 });
    let board = Board::new(BOARD_WIDTH, BOARD_HEIGHT, snake, food);

    // Updates and renders board
    let mut frame_buffer = ['.'; (BOARD_WIDTH * BOARD_HEIGHT) as usize];
    board.render_chars(&mut frame_buffer, CharPalette::default());
    print_frame_buffer(&frame_buffer, BOARD_WIDTH, BOARD_HEIGHT);
}


fn print_frame_buffer(frame_buffer: &[char], width: u32, height: u32) {
    let mut i = 0;
    for _ in 0..height {
        for _ in 0..width {
            let c = frame_buffer[i];
            print!("{c}{SEPARATION_CHAR}");
            i += 1;
        }
        println!();
    }
}