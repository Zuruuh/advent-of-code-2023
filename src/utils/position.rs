#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.x, self.y)
    }
}

impl Position {
    pub fn surrounding(&self) -> [Self; 8] {
        [
            self.next_x(),
            self.prev_x(),
            self.next_y(),
            self.prev_y(),
            self.next_x().next_y(),
            self.next_x().prev_y(),
            self.prev_x().prev_y(),
            self.prev_x().next_y(),
        ]
    }

    pub fn surrounding_without_diagonals(&self) -> [Self; 4] {
        [self.next_x(), self.next_y(), self.prev_x(), self.prev_y()]
    }

    pub fn prev_x(self) -> Self {
        self.with_x((self.x as i32 - 1).max(0) as usize)
    }

    pub fn prev_y(self) -> Self {
        self.with_y((self.y as i32 - 1).max(0) as usize)
    }

    pub fn next_x(self) -> Self {
        self.with_x(self.x + 1)
    }

    pub fn next_y(self) -> Self {
        self.with_y(self.y + 1)
    }

    pub fn with_x(self, x: usize) -> Self {
        Self { x, y: self.y }
    }

    pub fn with_y(self, y: usize) -> Self {
        Self { x: self.x, y }
    }
}
