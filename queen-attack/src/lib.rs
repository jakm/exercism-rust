#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank >= 0 && rank < 8 && file >= 0 && file < 8 {
            Some(Self(rank, file))
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
        if self.position.0 == other.position.0 || self.position.1 == other.position.1 {
            return true;
        }
        let x = (other.position.0 - self.position.0) as f32;
        let y = (other.position.1 - self.position.1) as f32;
        let tg_alpha = x / y;

        if tg_alpha == 1.0 || tg_alpha == -1.0 {
            true
        } else {
            false
        }
    }
}
