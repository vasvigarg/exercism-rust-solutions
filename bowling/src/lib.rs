#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self { rolls: Vec::new() }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_game_complete() {
            return Err(Error::GameComplete);
        }

        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if !self.is_valid_roll(pins) {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.rolls.push(pins);
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.can_score() {
            return None;
        }

        let mut total = 0;
        let mut roll_idx = 0;

        for _ in 0..9 {
            if roll_idx >= self.rolls.len() {
                return None;
            }

            if self.rolls[roll_idx] == 10 {
                if roll_idx + 2 >= self.rolls.len() {
                    return None;
                }
                total += 10 + self.rolls[roll_idx + 1] + self.rolls[roll_idx + 2];
                roll_idx += 1;
            } else {
                if roll_idx + 1 >= self.rolls.len() {
                    return None;
                }
                let sum = self.rolls[roll_idx] + self.rolls[roll_idx + 1];
                if sum == 10 {
                    if roll_idx + 2 >= self.rolls.len() {
                        return None;
                    }
                    total += 10 + self.rolls[roll_idx + 2];
                } else {
                    total += sum;
                }
                roll_idx += 2;
            }
        }

        while roll_idx < self.rolls.len() {
            total += self.rolls[roll_idx];
            roll_idx += 1;
        }

        Some(total)
    }

    fn is_valid_roll(&self, pins: u16) -> bool {
        let frame_info = self.get_frame_info();

        match frame_info {
            FrameInfo::RegularFrame { roll_in_frame, .. } => {
                if roll_in_frame == 1 {
                    let first = self.rolls[self.rolls.len() - 1];
                    first + pins <= 10
                } else {
                    true
                }
            }
            FrameInfo::TenthFrame { roll_in_frame } => {
                match roll_in_frame {
                    0 => true,
                    1 => {
                        let first = self.rolls[self.rolls.len() - 1];
                        first == 10 || first + pins <= 10
                    }
                    2 => {
                        let len = self.rolls.len();
                        let first = self.rolls[len - 2];
                        let second = self.rolls[len - 1];

                        if first == 10 {
                            if second == 10 {
                                true
                            } else {
                                second + pins <= 10
                            }
                        } else if first + second == 10 {
                            pins <= 10
                        } else {
                            false
                        }
                    }
                    _ => false,
                }
            }
        }
    }

    fn get_frame_info(&self) -> FrameInfo {
        let mut roll_idx = 0;
        let mut frame = 1;

        while frame <= 9 && roll_idx < self.rolls.len() {
            if self.rolls[roll_idx] == 10 {
                roll_idx += 1;
                frame += 1;
            } else if roll_idx + 1 < self.rolls.len() {
                roll_idx += 2;
                frame += 1;
            } else {
                return FrameInfo::RegularFrame { frame, roll_in_frame: 1 };
            }
        }

        if frame <= 9 {
            FrameInfo::RegularFrame { frame, roll_in_frame: 0 }
        } else {
            FrameInfo::TenthFrame {
                roll_in_frame: self.rolls.len() - roll_idx,
            }
        }
    }

    fn is_game_complete(&self) -> bool {
        let mut roll_idx = 0;

        for _ in 0..9 {
            if roll_idx >= self.rolls.len() {
                return false;
            }

            if self.rolls[roll_idx] == 10 {
                roll_idx += 1;
            } else {
                if roll_idx + 1 >= self.rolls.len() {
                    return false;
                }
                roll_idx += 2;
            }
        }

        if roll_idx >= self.rolls.len() {
            return false;
        }

        let rest = &self.rolls[roll_idx..];
        match rest.len() {
            0 | 1 => false,
            2 => {
                let first = rest[0];
                let second = rest[1];
                first != 10 && first + second < 10
            }
            3 => true,
            _ => true,
        }
    }

    fn can_score(&self) -> bool {
        let mut roll_idx = 0;

        for _ in 0..9 {
            if roll_idx >= self.rolls.len() {
                return false;
            }

            if self.rolls[roll_idx] == 10 {
                if roll_idx + 2 >= self.rolls.len() {
                    return false;
                }
                roll_idx += 1;
            } else {
                if roll_idx + 1 >= self.rolls.len() {
                    return false;
                }

                let sum = self.rolls[roll_idx] + self.rolls[roll_idx + 1];

                if sum == 10 {
                    if roll_idx + 2 >= self.rolls.len() {
                        return false;
                    }
                }
                roll_idx += 2;
            }
        }

        if roll_idx >= self.rolls.len() {
            return false;
        }

        let rest = &self.rolls[roll_idx..];
        match rest.len() {
            0 | 1 => false,
            2 => {
                let first = rest[0];
                let second = rest[1];
                first + second < 10
            }
            3 => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
enum FrameInfo {
    RegularFrame { frame: usize, roll_in_frame: usize },
    TenthFrame { roll_in_frame: usize },
}
