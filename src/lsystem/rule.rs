use super::LsystemAction;

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
