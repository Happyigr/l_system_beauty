use bevy::{math::Vec2, utils::HashMap};

use super::rule::Rule;

const TURN_LEFT_ANGLE: f32 = 90.0;
const TURN_RIGHT_ANGLE: f32 = 90.0;
const START_ANGLE: f32 = 90.0;
const LINE_LENGTH: f32 = 10.0;
const GROW_SCALING: f32 = 1.0;
const START_POINT: Vec2 = Vec2::new(0., 0.);

#[derive(Clone, Debug)]
struct Vec2Branched {
    point: Vec2,
    branches: Option<Vec<usize>>,
}
impl Vec2Branched {
    fn new(x: f32, y: f32) -> Self {
        Self {
            branches: None,
            point: Vec2::new(x, y),
        }
    }

    fn add_branch(&mut self, branch_id: usize) {
        self.branches.get_or_insert_with(Vec::new).push(branch_id);
    }
}

#[derive(Debug)]
pub struct LsystemTree {
    branches: HashMap<usize, Vec<Vec2Branched>>,
    branches_amount: usize,
}

impl LsystemTree {
    fn new() -> Self {
        Self {
            branches: HashMap::new(),
            branches_amount: 0,
        }
    }

    fn add_branch(&mut self, branch_id: usize, points: Vec<Vec2Branched>) {
        self.branches.insert(branch_id, points);
        self.branches_amount += 1;
    }
}

#[derive(Copy, Clone)]
pub enum LsystemAction {
    DrawForward,
    BranchStart,
    BranchEnd,
    TurnLeft,
    TurnRight,
}

// grow scaling will descend the length of the next line
pub struct Lsystem2Points {
    rules: HashMap<char, LsystemAction>,
    grow_scaling: f32,
    line_length: f32,
    start_angle: f32,
    start_point: Vec2Branched,
    turn_left_angle: f32,
    turn_right_angle: f32,
}

impl Lsystem2Points {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
            grow_scaling: GROW_SCALING,
            line_length: LINE_LENGTH,
            start_angle: START_ANGLE,
            start_point: Vec2Branched::new(START_POINT.x, START_POINT.y),
            turn_left_angle: TURN_LEFT_ANGLE,
            turn_right_angle: TURN_RIGHT_ANGLE,
        }
    }

    pub fn add_rule(&mut self, rule: &Rule) {
        self.rules.insert(rule.ch, rule.action.clone());
    }

    pub fn build_tree(&self, lsystem: String) -> Result<LsystemTree, String> {
        let mut tree = LsystemTree::new();

        let mut current_branch: Vec<Vec2Branched> = vec![];
        // let mut current_branch_id: usize = 0;

        let mut current_angle = self.start_angle;
        let mut current_point = self.start_point.clone();

        for ch in lsystem.chars() {
            println!("{current_angle}");
            if let Some(action) = self.rules.get(&ch) {
                match action {
                    LsystemAction::DrawForward => {
                        let current_dir = Vec2::new(
                            current_angle.to_radians().cos(),
                            current_angle.to_radians().sin(),
                        );

                        current_branch.push(current_point.clone());
                        current_point.point += current_dir * self.line_length;
                    }
                    LsystemAction::TurnLeft => current_angle += self.turn_left_angle,
                    LsystemAction::TurnRight => current_angle -= self.turn_right_angle,
                    LsystemAction::BranchStart => todo!(),
                    LsystemAction::BranchEnd => todo!(),
                }
            } else {
                return Err(format!("Action for {ch} is not found"));
            }
        }
        tree.add_branch(0, current_branch);

        Ok(tree)
    }
}
