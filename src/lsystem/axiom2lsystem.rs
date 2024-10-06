use std::collections::HashMap;

use super::rule::Rule;

pub struct Axiom2Lsystem {
    axiom: String,
    rules: HashMap<char, String>,
}

impl Axiom2Lsystem {
    pub fn new(axiom: String) -> Self {
        Self {
            axiom,
            rules: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, rule: &Rule) {
        self.rules.insert(rule.ch, rule.to.clone());
    }

    pub fn build(&self, lvl: usize) -> Result<String, String> {
        let mut current_sequence = self.axiom.clone();

        for _ in 0..lvl {
            let mut temp = "".to_string();

            for ch in current_sequence.chars() {
                if let Some(new_str) = &self.rules.get(&ch) {
                    temp.push_str(new_str);
                } else {
                    return Err(format!("{ch} not found in rules"));
                }
            }

            current_sequence = temp;
        }

        Ok(current_sequence)
    }
}
