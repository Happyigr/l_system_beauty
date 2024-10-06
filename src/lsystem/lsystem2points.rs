use bevy::{math::Vec2, utils::HashMap};

use super::rule::Rule;

const TURN_LEFT_ANGLE: f32 = 90.0;
const TURN_RIGHT_ANGLE: f32 = 90.0;
const START_ANGLE: f32 = 90.0;
const LINE_LENGTH: f32 = 10.0;
const GROW_SCALING: f32 = 1.0;
const START_POINT: Vec2 = Vec2::new(0., 0.);
const CLOSING_BRACKET: char = ']';
const OPEN_BRACKET: char = '[';

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

        Ok(tree)
    }

    fn build_branch(&self, lsystem: String, branch_id: usize, tree: &mut LsystemTree) {
        let mut current_branch: Vec<Vec2Branched> = vec![];

        let mut current_angle = self.start_angle;
        let mut current_point = self.start_point.clone();

        for (i, ch) in lsystem.chars().enumerate() {
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
                panic!("Action for {ch} is not found");
            }
        }
    }
}

fn find_branch_opening_bracket(lsystem: &String, branch_id: usize) -> usize {
    let mut last_opened_branch_id: usize = 0;
    let mut queued_branches: Vec<usize> = vec![];
    let mut current_branch = 0;

    if branch_id == 0 {
        return 0;
    }

    for (i, ch) in lsystem.chars().enumerate() {
        match ch {
            OPEN_BRACKET => {
                queued_branches.push(current_branch);
                last_opened_branch_id += 1;
                current_branch = last_opened_branch_id;

                if current_branch == branch_id {
                    return i;
                }
            }
            CLOSING_BRACKET => {
                current_branch = queued_branches.pop().unwrap();
            }
            _ => {}
        }
    }

    panic!("Failed to find branch opening bracket");
}

fn find_branch_closing_bracket(lsystem: &String, branch_id: usize) -> usize {
    let mut last_opened_branch_id: usize = 0;
    let mut queued_branches: Vec<usize> = vec![];
    let mut current_branch = 0;

    for (i, ch) in lsystem.chars().enumerate() {
        match ch {
            OPEN_BRACKET => {
                queued_branches.push(current_branch);
                last_opened_branch_id += 1;
                current_branch = last_opened_branch_id;
            }
            CLOSING_BRACKET => {
                if current_branch == branch_id {
                    return i;
                }
                current_branch = queued_branches.pop().unwrap();
            }
            _ => {}
        }
    }

    // if it the first branch, then the end will be on the end of the string
    return lsystem.len() - 1;
}
