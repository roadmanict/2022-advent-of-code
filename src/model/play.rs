pub enum PlayResult {
    Win,
    Draw,
    Loss,
}
impl PlayResult {
    pub fn score(&self) -> usize {
        match &self {
            PlayResult::Win => return 6,
            PlayResult::Draw => return 3,
            PlayResult::Loss => return 0,
        }
    }
}
pub enum Play {
    Rock,
    Paper,
    Scissors,
}
impl Play {
    pub fn from_opponent(play: &str) -> Play {
        if play.eq("A") {
            return Play::Rock;
        } else if play.eq("B") {
            return Play::Paper;
        } else {
            return Play::Scissors;
        }
    }

    pub fn from_answer(play: &str) -> Play {
        if play.eq("X") {
            return Play::Rock;
        } else if play.eq("Y") {
            return Play::Paper;
        } else {
            return Play::Scissors;
        }
    }
    pub fn score(&self) -> usize {
        match &self {
            Play::Rock => return 1,
            Play::Paper => return 2,
            Play::Scissors => return 3,
        }
    }

    pub fn compare(&self, other_play: Play) -> PlayResult {
        match &self {
            Play::Rock => match other_play {
                Play::Rock => return PlayResult::Draw,
                Play::Paper => return PlayResult::Loss,
                Play::Scissors => return PlayResult::Win,
            },
            Play::Paper => match other_play {
                Play::Rock => return PlayResult::Win,
                Play::Paper => return PlayResult::Draw,
                Play::Scissors => return PlayResult::Loss,
            },
            Play::Scissors => match other_play {
                Play::Rock => return PlayResult::Loss,
                Play::Paper => return PlayResult::Win,
                Play::Scissors => return PlayResult::Draw,
            },
        }
    }
}
