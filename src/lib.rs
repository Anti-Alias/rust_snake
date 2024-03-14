use std::io::Stdout;
use std::ops::Add;

use crossterm::cursor::MoveTo;
use crossterm::style::Print;
use crossterm::QueueableCommand;

/// A board that contains the game of snake.
pub struct Board {
    width: u32,
    height: u32,
    snake: Snake,
    food: Food,
    state: GameState,
}

impl Board {

    /// Allocates game board
    pub fn new(width: u32, height: u32, snake: Snake, food: Food) -> Self {
        if width == 0 || height == 0 { panic!("Invalid board size {width}x{height}") }
        Self {
            width,
            height,
            snake,
            food,
            state: GameState::Running,
        }
    }

    /// Renders game board to stdout.
    pub fn render_to_stdout(&self, stdout: &mut Stdout, palette: CharPalette) {

        // Clears screen
        for y in 0..self.height {
            stdout.queue(MoveTo(0, y as u16)).unwrap();
            for _ in 0..self.width {
                stdout.queue(Print(palette.empty_char)).unwrap();
                stdout.queue(Print(palette.clear_char)).unwrap();
            }
        };

        // Renders snake
        for snake_pos in self.snake.positions.iter().copied() {
            if self.is_in_bounds(snake_pos) {
                stdout.queue(MoveTo(snake_pos.x as u16 * 2, snake_pos.y as u16)).unwrap();
                stdout.queue(Print(palette.snake_char)).unwrap();
                stdout.queue(Print(palette.clear_char)).unwrap();
            }
        }

        // Renders food
        let food_pos = self.food.position;
        if  self.is_in_bounds(food_pos) {
            stdout.queue(MoveTo(food_pos.x as u16 * 2, food_pos.y as u16)).unwrap();
            stdout.queue(Print(palette.food_char)).unwrap();
            stdout.queue(Print(palette.clear_char)).unwrap();
        }
    }

    pub fn update(&mut self) {
        let snake_head_pos = &mut self.snake.positions[0];
        *snake_head_pos = *snake_head_pos + self.snake.direction.to_delta();
        for i in 1..self.snake.positions.len()-1 {
            self.snake.positions[i] = self.snake.positions[i-1];
        }
    }

    /// Width and height of board.
    pub fn size(&self) -> (u32, u32) { (self.width, self.height) }

    /// State of the game
    pub fn state(&self) -> GameState { self.state }

    /// Converts a board position into an index into the frame buffer.
    /// Returns [None] if out of bounds.
    fn is_in_bounds(&self, pos: Vec2) -> bool {
        pos.x >= 0 && pos.x < self.width as i32 && pos.y >= 0 && pos.y < self.height as i32
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

/// Direction the snake is facing.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Direction { N, S, E, W }

impl Direction {
    pub fn to_delta(self) -> Vec2 {
        match self {
            Direction::N => Vec2 { x: 0, y: -1 },
            Direction::S => Vec2 { x: 0, y: 1 },
            Direction::E => Vec2 { x: 1, y: 0 },
            Direction::W => Vec2 { x: -1, y: 0 },
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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GameState {
    Running,
    Dead,
}