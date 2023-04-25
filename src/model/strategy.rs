use std::str::FromStr;

use crate::model::play::Play;

use super::play::PlayResultStrategy;

pub struct Strategy {
    pub opponent: Play,
    pub answer: Play,
    pub play_result_strategy: PlayResultStrategy,
}

impl Strategy {
    pub fn new(opponent: Play, answer: Play, play_result_strategy: PlayResultStrategy) -> Strategy {
        Strategy {
            opponent,
            answer,
            play_result_strategy,
        }
    }
}

impl FromStr for Strategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (input, answer) = match s.split_once(' ') {
            Some(it) => it,
            None => return Err(format!("Could not parse strategy string: {}", s)),
        };

        Ok(Strategy::new(
            Play::from_opponent(input),
            Play::from_answer(answer),
            PlayResultStrategy::from_answer(answer),
        ))
    }
}
