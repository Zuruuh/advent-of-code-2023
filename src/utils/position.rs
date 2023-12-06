#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn surrounding(&self) -> [Position; 8] {
        let x = self.x;
        let y = self.y;

        [
            Self { x, y: y + 1 },
            Self {
                x,
                y: (y as i32 - 1).max(0) as usize,
            },
            Self { x: x + 1, y: y + 1 },
            Self {
                x: x + 1,
                y: (y as i32 - 1).max(0) as usize,
            },
            Self {
                x: (x as i32 - 1).max(0) as usize,
                y: (y as i32 - 1).max(0) as usize,
            },
            Self {
                x: (x as i32 - 1).max(0) as usize,
                y: y + 1,
            },
            Self {
                x: (x as i32 - 1).max(0) as usize,
                y,
            },
            Self { x: x + 1, y },
        ]
    }
}
