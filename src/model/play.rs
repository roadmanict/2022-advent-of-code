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
}
