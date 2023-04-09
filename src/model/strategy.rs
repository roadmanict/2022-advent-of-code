use crate::model::play::Play;

pub struct Strategy {
    pub opponent: Play,
    pub answer: Play,
}
impl Strategy {
    pub fn new(opponent: Play, answer: Play) -> Strategy {
        Strategy { opponent, answer }
    }
}
