#[derive(Debug, PartialEq, Eq)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if (0..8).contains(&rank) && (0..8).contains(&file) {
            Some(Self { rank, file })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let (r1, f1) = (self.position.rank, self.position.file);
        let (r2, f2) = (other.position.rank, other.position.file);

        // Same row or column
        r1 == r2 || f1 == f2 ||
        // Same diagonal
        (r1 - r2).abs() == (f1 - f2).abs()
    }
}
