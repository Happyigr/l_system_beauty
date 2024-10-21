use bevy::{math::Vec2, utils::HashMap};

use super::rule::{LsystemAction, Rule};

const TURN_LEFT_ANGLE: f32 = 20.0;
const TURN_RIGHT_ANGLE: f32 = 80.0;
const START_ANGLE: f32 = 90.0;
const LINE_LENGTH: f32 = 10.0;
const GROW_SCALING: f32 = 1.0;
const START_POINT: Vec2 = Vec2::new(0., 0.);

#[derive(Debug)]
pub struct LsystemTree {
    pub branches: HashMap<usize, Vec<Vec2Branched>>,
    pub branches_amount: usize,
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

#[derive(Clone, Debug)]
pub struct Vec2Branched {
    pub point: Vec2,
    pub branches: Option<Vec<usize>>,
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

        let mut current_state = MyState::new(
            0,
            vec![self.start_point.clone()],
            self.start_point.clone(),
            self.start_angle,
        );

        let mut queued_states: Vec<MyState> = vec![];
        let mut last_created_branch: usize = 0;

        for ch in lsystem.chars() {
            if let Some(action) = self.rules.get(&ch) {
                match action {
                    LsystemAction::DrawForward => {
                        // calculate the normilized dir from angle
                        let current_dir = Vec2::new(
                            current_state.angle.to_radians().cos(),
                            current_state.angle.to_radians().sin(),
                        );

                        // add it to current points
                        current_state.points.push(current_state.next_point.clone());
                        // change next point
                        current_state.next_point.point += current_dir * self.line_length;
                        // if there were some branches connected to the point remove them
                        current_state.next_point.branches = None;
                    }
                    LsystemAction::TurnLeft => current_state.angle += self.turn_left_angle,
                    LsystemAction::TurnRight => current_state.angle -= self.turn_right_angle,
                    LsystemAction::BranchStart => {
                        current_state.next_point.add_branch(last_created_branch + 1);

                        queued_states.push(current_state.clone());
                        current_state.points = vec![];
                        current_state.id = last_created_branch + 1;
                        current_state.next_point.branches = None;
                        last_created_branch += 1;
                    }
                    LsystemAction::BranchEnd => {
                        tree.add_branch(current_state.id, current_state.points);
                        current_state = queued_states.pop().unwrap();
                    }
                }
            } else {
                panic!("Action for {ch} is not found");
            }
        }

        tree.add_branch(current_state.id, current_state.points);

        Ok(tree)
    }
}

// help structures
#[derive(Clone)]
struct MyState {
    id: usize,
    points: Vec<Vec2Branched>,
    next_point: Vec2Branched,
    angle: f32,
}

impl MyState {
    fn new(id: usize, points: Vec<Vec2Branched>, last_point: Vec2Branched, angle: f32) -> Self {
        Self {
            id,
            points,
            next_point: last_point,
            angle,
        }
    }
}
