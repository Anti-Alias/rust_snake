use std::io::{stdout, Write};
use std::time::Duration;
use crossterm::cursor::{Hide, MoveTo};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::terminal::{Clear, ClearType};
use crossterm::QueueableCommand;
use snake::{Board, CharPalette, Input, Direction, Food, Snake, Vec2};

const SLEEP_TIME: Duration = Duration::from_millis(150);
const BOARD_WIDTH: u32 = 18;
const BOARD_HEIGHT: u32 = 16;

fn main() {

    // Creates snake board.
    let snake = Snake::new(Vec2 { x: 0, y: 0 }, Direction::S);
    let food = Food::new(Vec2 { x: 5, y: 5 });
    let mut board = Board::new(BOARD_WIDTH, BOARD_HEIGHT, snake, food);

    // Clears terminal and hides cursor
    let mut stdout = stdout();
    stdout.queue(Clear(ClearType::All)).unwrap();
    stdout.queue(Hide).unwrap();
    stdout.flush().unwrap();

    // Renders board and updates it
    loop {
        print_board(&mut board);
        std::thread::sleep(SLEEP_TIME);
        let input = read_keyboard_input();
        board.update(input);
        if board.quit() { break }
    }
}

fn print_board(board: &mut Board) {
    let mut frame_buffer = [' '; BOARD_WIDTH as usize * BOARD_HEIGHT as usize];
    board.render(&mut frame_buffer, CharPalette::default());
    print_frame_buffer(&frame_buffer, BOARD_WIDTH, BOARD_HEIGHT);
}

fn print_frame_buffer(chars: &[char], width: u32, height: u32) {
    let mut stdout = stdout();
    stdout.queue(MoveTo(0, 0)).unwrap();
    let mut i = 0;
    for _ in 0..height {
        for _ in 0..width {
            let c = chars[i];
            write!(stdout, "{c} ").unwrap();
            i += 1;
        }
        write!(stdout, "\n").unwrap();
    }
    stdout.flush().unwrap();
}

fn read_keyboard_input() -> Option<Input> {
    let mut result = None;
    while crossterm::event::poll(Duration::ZERO).unwrap() {
        match crossterm::event::read().unwrap() {
            Event::Key(key_event) => match key_event {
                KeyEvent { code: KeyCode::Up, kind: KeyEventKind::Press, .. } => result = Some(Input::Face(Direction::N)),
                KeyEvent { code: KeyCode::Down, kind: KeyEventKind::Press, .. } => result = Some(Input::Face(Direction::S)),
                KeyEvent { code: KeyCode::Right, kind: KeyEventKind::Press, .. } => result = Some(Input::Face(Direction::E)),
                KeyEvent { code: KeyCode::Left, kind: KeyEventKind::Press, .. } => result = Some(Input::Face(Direction::W)),
                KeyEvent { code: KeyCode::Char('w'), kind: KeyEventKind::Press, .. } => result = Some(Input::Face(Direction::N)),
                KeyEvent { code: KeyCode::Char('s'), kind: KeyEventKind::Press, .. } => result = Some(Input::Face(Direction::S)),
                KeyEvent { code: KeyCode::Char('d'), kind: KeyEventKind::Press, .. } => result = Some(Input::Face(Direction::E)),
                KeyEvent { code: KeyCode::Char('a'), kind: KeyEventKind::Press, .. } => result = Some(Input::Face(Direction::W)),
                KeyEvent { code: KeyCode::Char('q'), kind: KeyEventKind::Press, .. } => result = Some(Input::Quit),
                KeyEvent { code: KeyCode::Char('r'), kind: KeyEventKind::Press, .. } => result = Some(Input::Retry),
                _ => {}
            },
            _ => {}
        };
    }
    result
}