use std::io::{stdout, Stdout, Write};
use std::time::Duration;
use crossterm::cursor::{Hide, MoveTo};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::QueueableCommand;
use snake::{Board, CharPalette, Input, Direction, Food, Snake, State, Vec2};

const SLEEP_TIME: Duration = Duration::from_millis(150);
const BOARD_WIDTH: u32 = 31;
const BOARD_HEIGHT: u32 = 21;

fn main() {

    // Creates snake board.
    let snake = Snake::new(Vec2 { x: 0, y: 0 }, Direction::S);
    let food = Food::new(Vec2 { x: 5, y: 5 });
    let mut board = Board::new(BOARD_WIDTH, BOARD_HEIGHT, snake, food);

    // Hides cursor and clears terminal
    let mut stdout = stdout();
    stdout
        .queue(Clear(ClearType::All)).unwrap()
        .queue(Hide).unwrap()
        .flush().unwrap();
    

    // Renders board and updates it
    let mut frame_buffer = [' '; BOARD_WIDTH as usize * BOARD_HEIGHT as usize];
    loop {
        print_board(&mut board, &mut frame_buffer);
        std::thread::sleep(SLEEP_TIME);
        update_board(&mut board);
        if board.state() == State::Quit {
            break
        }
    }
}

fn print_board(board: &mut Board, frame_buffer: &mut [char]) {
    let mut stdout = stdout();
    board.render(frame_buffer, CharPalette::default());
    print_frame_buffer(&mut stdout, &frame_buffer, BOARD_WIDTH, BOARD_HEIGHT);
}

fn update_board(board: &mut Board) {
    let mut input = None;
    while let Some(inp) = read_keyboard_input() {
        input = Some(inp);
    }
    board.update(input);
}

fn read_keyboard_input() -> Option<Input> {
    if poll(Duration::ZERO).unwrap() {
        match read().unwrap() {
            Event::Key(key_event) => match key_event {
                KeyEvent { code: KeyCode::Up, kind: KeyEventKind::Press, .. } => return Some(Input::Face(Direction::N)),
                KeyEvent { code: KeyCode::Down, kind: KeyEventKind::Press, .. } => return Some(Input::Face(Direction::S)),
                KeyEvent { code: KeyCode::Right, kind: KeyEventKind::Press, .. } => return Some(Input::Face(Direction::E)),
                KeyEvent { code: KeyCode::Left, kind: KeyEventKind::Press, .. } => return Some(Input::Face(Direction::W)),
                KeyEvent { code: KeyCode::Char('w'), kind: KeyEventKind::Press, .. } => return Some(Input::Face(Direction::N)),
                KeyEvent { code: KeyCode::Char('s'), kind: KeyEventKind::Press, .. } => return Some(Input::Face(Direction::S)),
                KeyEvent { code: KeyCode::Char('d'), kind: KeyEventKind::Press, .. } => return Some(Input::Face(Direction::E)),
                KeyEvent { code: KeyCode::Char('a'), kind: KeyEventKind::Press, .. } => return Some(Input::Face(Direction::W)),
                KeyEvent { code: KeyCode::Char('q'), kind: KeyEventKind::Press, .. } => return Some(Input::Quit),
                KeyEvent { code: KeyCode::Char('r'), kind: KeyEventKind::Press, .. } => return Some(Input::Retry),
                _ => {}
            },
            _ => {}
        };
    }
    None
}

fn print_frame_buffer(stdout: &mut Stdout, chars: &[char], width: u32, height: u32) {
    let mut i = 0;
    let mut string = String::new();
    for _ in 0..height {
        for _ in 0..width {
            let c = chars[i];
            string.push(c);
            string.push(' ');
            i += 1;
        }
        string.push('\n');
    }
    stdout
        .queue(MoveTo(0, 0)).unwrap()
        .queue(Print(string)).unwrap();
}