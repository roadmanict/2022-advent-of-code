#[derive(Debug)]
pub enum PlayResult {
    Win,
    Draw,
    Loss,
}
impl PlayResult {
    pub fn score(&self) -> usize {
        match &self {
            PlayResult::Win => 6,
            PlayResult::Draw => 3,
            PlayResult::Loss => 0,
        }
    }
}

#[derive(Debug)]
pub enum PlayResultStrategy {
    Win,
    Draw,
    Lose,
}

impl PlayResultStrategy {
    pub fn from_answer(play: &str) -> PlayResultStrategy {
        if play.eq("X") {
            PlayResultStrategy::Lose
        } else if play.eq("Y") {
            return PlayResultStrategy::Draw;
        } else {
            return PlayResultStrategy::Win;
        }
    }
    pub fn should_play(&self, play: &Play) -> Play {
        match &self {
            PlayResultStrategy::Win => play.loses_from(),
            PlayResultStrategy::Draw => play.draws_from(),
            PlayResultStrategy::Lose => play.wins_from(),
        }
    }
}

#[derive(Debug)]
pub enum Play {
    Rock,
    Paper,
    Scissors,
}
impl Play {
    pub fn from_opponent(play: &str) -> Play {
        if play.eq("A") {
            Play::Rock
        } else if play.eq("B") {
            return Play::Paper;
        } else {
            return Play::Scissors;
        }
    }

    pub fn from_answer(play: &str) -> Play {
        if play.eq("X") {
            Play::Rock
        } else if play.eq("Y") {
            return Play::Paper;
        } else {
            return Play::Scissors;
        }
    }
    pub fn score(&self) -> usize {
        match &self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }
    pub fn draws_from(&self) -> Play {
        match &self {
            Play::Rock => Play::Rock,
            Play::Paper => Play::Paper,
            Play::Scissors => Play::Scissors,
        }
    }

    pub fn wins_from(&self) -> Play {
        match &self {
            Play::Rock => Play::Scissors,
            Play::Paper => Play::Rock,
            Play::Scissors => Play::Paper,
        }
    }

    pub fn loses_from(&self) -> Play {
        match &self {
            Play::Rock => Play::Paper,
            Play::Paper => Play::Scissors,
            Play::Scissors => Play::Rock,
        }
    }

    pub fn versus(&self, other_play: &Play) -> PlayResult {
        match &self {
            Play::Rock => match other_play {
                Play::Rock => PlayResult::Draw,
                Play::Paper => PlayResult::Loss,
                Play::Scissors => PlayResult::Win,
            },
            Play::Paper => match other_play {
                Play::Rock => PlayResult::Win,
                Play::Paper => PlayResult::Draw,
                Play::Scissors => PlayResult::Loss,
            },
            Play::Scissors => match other_play {
                Play::Rock => PlayResult::Loss,
                Play::Paper => PlayResult::Win,
                Play::Scissors => PlayResult::Draw,
            },
        }
    }
}
