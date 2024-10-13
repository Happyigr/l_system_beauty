#[derive(Copy, Clone)]
pub enum LsystemAction {
    DrawForward,
    BranchStart,
    BranchEnd,
    TurnLeft,
    TurnRight,
}

pub struct Rule {
    pub ch: char,
    pub to: String,
    pub action: LsystemAction,
}

impl Rule {
    pub fn new(ch: char, to: String, action: LsystemAction) -> Self {
        Self { ch, to, action }
    }
}
