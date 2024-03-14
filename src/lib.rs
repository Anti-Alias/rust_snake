/// A board that contains the game of snake.
pub struct Board {
    width: u32,
    height: u32,
    snake: Snake,
    food: Food,
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
        }
    }

    /// Renders game board to internal frame buffer.
    /// First, clears frame buffer.
    /// Then, renders snake.
    /// Then, renders food character.
    pub fn render_chars(&self, frame_buffer: &mut [char], palette: CharPalette) {
        if frame_buffer.len() != (self.width * self.height) as usize {
            panic!("Incorrect frame buffer size");
        }
        for c in frame_buffer.iter_mut() {
            *c = palette.clear_char;
        }
        for snake_pos in self.snake.positions.iter().copied() {
            if let Some(index) = self.index_of_pos(snake_pos) {
                frame_buffer[index] = palette.snake_char;
            }
        }
        let food_pos = self.food.position;
        if let Some(index) = self.index_of_pos(food_pos) {
            frame_buffer[index] = palette.food_char;
        }
    }

    /// Width and height of board.
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Converts a board position into an index into the frame buffer.
    /// Returns [None] if out of bounds.
    fn index_of_pos(&self, pos: Position) -> Option<usize> {
        if pos.x < 0 || pos.y < 0 || pos.x as u32 >= self.width || pos.y as u32 >= self.height {
            return None;
        }
        let (x, y, w) = (pos.x as usize, pos.y as usize, self.width as usize);
        Some(y*w + x)
    }
}

/// Snake game object.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Snake {
    positions: Vec<Position>,
    direction: Direction,
}

impl Snake {
    pub fn new(position: Position, direction: Direction) -> Self {
        Self {
            positions: vec![position],
            direction,
        }
    }
}

/// Food game object.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Food {
    pub position: Position
}

impl Food {
    pub fn new(position: Position) -> Self {
        Self { position }
    }
}

/// Primitive 2D position for game objects.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

/// Direction the snake is facing.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Direction { N, S, E, W }

/// Palette to use when rendering characters.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct CharPalette {
    pub clear_char: char,
    pub snake_char: char,
    pub food_char: char,
}

impl Default for CharPalette {
    fn default() -> Self {
        Self {
            clear_char: '.',
            snake_char: 'S',
            food_char: 'F',
        }
    }
}