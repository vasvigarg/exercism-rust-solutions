#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    /// Construct a new `HighScores` instance from a slice of scores
    pub fn new(scores: &[u32]) -> Self {
        Self {
            scores: scores.to_vec(),
        }
    }

    /// Return all scores as a slice
    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    /// Return the latest (last) score, or `None` if empty
    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    /// Return the personal best (maximum score), or `None` if empty
    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().copied().max()
    }

    /// Return the top three scores in descending order
    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top = self.scores.clone();
        top.sort_unstable_by(|a, b| b.cmp(a)); 
        top.truncate(3);
        top
    }
}
