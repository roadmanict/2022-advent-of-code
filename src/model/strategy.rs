use crate::model::play::Play;

use super::play::PlayResultStrategy;

pub struct Strategy {
    pub opponent: Play,
    pub answer: Play,
    pub play_result_strategy: PlayResultStrategy,
}
impl Strategy {
    pub fn new(opponent: Play, answer: Play, play_result_strategy: PlayResultStrategy) -> Strategy {
        Strategy { opponent, answer, play_result_strategy }
    }
}
