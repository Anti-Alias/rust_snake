use std::ops::{Add, Sub};
use rand::prelude::*;

const GAMEOVER_TEXT_LINE_1: &str = "Game Over";
const GAMEOVER_TEXT_LINE_2: &str = "Gabe would have";
const GAMEOVER_TEXT_LINE_3: &str = "done better!";

/// A board that contains the game of snake.
pub struct Board {
    width: u32,
    height: u32,
    snake: Snake,
    food: Food,
    state: State,
    initial_state: InitialState,
}

impl Board {

    /// Allocates game board
    pub fn new(width: u32, height: u32, snake: Snake, food: Food) -> Self {
        if width == 0 || height == 0 { panic!("Invalid board size {width}x{height}") }
        let initial_state = InitialState {
            snake_pos: snake.positions[0],
            snake_dir: snake.direction,
            food_pos: food.position,
        };
        Self {
            width,
            height,
            snake,
            food,
            state: State::Running,
            initial_state,
        }
    }

    /// Renders game board to stdout.
    pub fn render(&self, chars: &mut [char], palette: CharPalette) {
        for c in chars.iter_mut() {
            *c = palette.empty_char;
        }
        match self.state {
            State::Running => self.render_running(chars, palette),
            State::GameOver => self.render_game_over(chars),
            State::Quit => panic!("Cannot render a game that quit"),
        }
    }

    fn render_running(&self, chars: &mut [char], palette: CharPalette) {
        for snake_pos in self.snake.positions.iter().copied() {
            if let Some(index) = self.index_of_pos(snake_pos) {
                chars[index] = palette.snake_char
            }
        }
        let food_pos = self.food.position;
        if let Some(index) = self.index_of_pos(food_pos) {
            chars[index] = palette.food_char;
        }
    }

    fn render_game_over(&self, chars: &mut [char]) {
        let center_x = self.width as i32 / 2;
        let line_1 = Vec2 {
            x: (center_x - GAMEOVER_TEXT_LINE_1.len() as i32 / 2 - 1).max(0) as i32,
            y: self.height as i32 / 2 - 2,
        };
        let line_2 = Vec2 {
            x: (center_x - GAMEOVER_TEXT_LINE_2.len() as i32 / 2 - 1).max(0) as i32,
            y: self.height as i32 / 2 + 2,
        };
        let line_3 = Vec2 {
            x: (center_x - GAMEOVER_TEXT_LINE_3.len() as i32 / 2 - 1).max(0) as i32,
            y: self.height as i32 / 2 + 3,
        };
        self.render_text(chars, GAMEOVER_TEXT_LINE_1, line_1);
        self.render_text(chars, GAMEOVER_TEXT_LINE_2, line_2);
        self.render_text(chars, GAMEOVER_TEXT_LINE_3, line_3);
        self.render_text(chars, "Press 'q' to quit", Vec2 { x: 0, y: self.height as i32 - 2});
        self.render_text(chars, "Press 'r' to retry", Vec2 { x: 0, y: self.height as i32 - 1});
    }

    fn render_text(&self, chars: &mut [char], text: &str, pos: Vec2) {
        for (i, c) in text.chars().enumerate() {
            let c_pos = Vec2 { x: pos.x + i as i32, y: pos.y };
            let Some(c_index) = self.index_of_pos(c_pos) else { return };
            chars[c_index] = c;
        }
    }

    pub fn update(&mut self, input: Option<Input>) {
        match self.state {
            State::Running => self.update_running(input),
            State::GameOver => self.update_game_over(input),
            State::Quit => panic!("Cannot update a game that quit"),
        }
    }

    fn update_running(&mut self, input: Option<Input>) {

        // Controls snake direction
        if let Some(Input::Face(direction)) = input {
            if direction.reverse() != self.snake.direction {
                self.snake.direction = direction;
            }
        }

        // Updates snake
        self.snake.update();
        let snake_head_pos = self.snake.head();
        if !self.in_bounds(snake_head_pos) || self.snake.is_colliding_self() {
            self.state = State::GameOver;
            return;
        }
        if snake_head_pos == self.food.position {
            self.snake.grow();
            let mut rng = thread_rng();
            self.food.position = Vec2 {
                x: (rng.gen::<u32>() % self.width) as i32,
                y: (rng.gen::<u32>() % self.height) as i32,
            };
        }
    }

    fn update_game_over(&mut self, input: Option<Input>) {
        match input {
            Some(Input::Retry) => self.retry(),
            Some(Input::Quit) => self.state = State::Quit,
            _ => {}
        }
    }

    // Restores game back to initial state after a game over.
    fn retry(&mut self) {
        self.state = State::Running;
        self.snake.positions.clear();
        self.snake.positions.push(self.initial_state.snake_pos);
        self.snake.direction = self.initial_state.snake_dir;
        self.food.position = self.initial_state.food_pos;
    }

    /// Width and height of board.
    pub fn size(&self) -> (u32, u32) { (self.width, self.height) }

    /// State of the game
    pub fn state(&self) -> State { self.state }

    /// Converts a board position into an index into the frame buffer.
    /// Returns [None] if out of bounds.
    fn index_of_pos(&self, pos: Vec2) -> Option<usize> {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.width as i32 || pos.y >= self.height as i32 {
            return None;
        }
        let (x, y, width) = (pos.x as usize, pos.y as usize, self.width as usize);
        Some(y * width + x)
    }

    fn in_bounds(&self, pos: Vec2) -> bool {
        pos.x >=0 && pos.y >= 0 && pos.x < self.width as i32 && pos.y < self.height as i32
    }
}

/// Snake game object.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Snake {
    positions: Vec<Vec2>,
    direction: Direction,
}

impl Snake {
    pub fn new(position: Vec2, direction: Direction) -> Self {
        Self {
            positions: vec![position],
            direction,
        }
    }

    fn update(&mut self) {
        for i in (1..self.positions.len()).rev() {
            self.positions[i] = self.positions[i-1];
        }
        let snake_head_pos = &mut self.positions[0];
        *snake_head_pos = *snake_head_pos + self.direction.to_delta();
    }

    fn is_colliding_self(&self) -> bool {
        let mut positions = self.positions.iter();
        let head = positions.next().unwrap();
        for segment in positions {
            if segment == head {
                return true;
            }
        }
        false
    }

    fn head(&self) -> Vec2 {
        self.positions[0]
    }

    fn grow(&mut self) {
        let tail_pos = self.positions[self.positions.len() - 1];
        self.positions.push(tail_pos);
    }
}

/// Food game object.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Food {
    pub position: Vec2
}

impl Food {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }
}

/// Primitive 2D vector for game objects.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

/// Direction the snake is facing.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Direction { N, S, E, W }

impl Direction {
    fn to_delta(self) -> Vec2 {
        match self {
            Direction::N => Vec2 { x: 0, y: -1 },
            Direction::S => Vec2 { x: 0, y: 1 },
            Direction::E => Vec2 { x: 1, y: 0 },
            Direction::W => Vec2 { x: -1, y: 0 },
        }
    }

    fn reverse(self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::N,
            Direction::E => Direction::W,
            Direction::W => Direction::E,
        }
    }
}

/// Palette to use when rendering characters.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct CharPalette {
    pub empty_char: char,
    pub clear_char: char,
    pub snake_char: char,
    pub food_char: char,
}

impl Default for CharPalette {
    fn default() -> Self {
        Self {
            empty_char: '.',
            clear_char: ' ',
            snake_char: 'S',
            food_char: 'F',
        }
    }
}

/// State of the game.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum State {
    Running,
    GameOver,
    Quit,
}

/// User input.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Input {
    Face(Direction),
    Retry,
    Quit,
}

/// Initial state of the game.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct InitialState {
    snake_pos: Vec2,
    snake_dir: Direction,
    food_pos: Vec2,
}