use bevy::prelude::*;
use lsystem::Axiom2Lsystem;

use crate::lsystem::{Lsystem2Points, LsystemAction, Rule};

mod lsystem;

fn main() {
    let mut app = App::new();

    app.run();

    let rules = vec![
        Rule::new(
            'X',
            "F+[[X]-X]-F[-FX]+X".to_string(),
            LsystemAction::DrawForward,
        ),
        Rule::new('F', "FF".to_string(), LsystemAction::DrawForward),
        Rule::new('[', "[".to_string(), LsystemAction::BranchStart),
        Rule::new(']', "]".to_string(), LsystemAction::BranchEnd),
        Rule::new('+', "+".to_string(), LsystemAction::TurnLeft),
        Rule::new('-', "-".to_string(), LsystemAction::TurnRight),
    ];

    let mut axiom2lsystem = Axiom2Lsystem::new("X".to_string());
    let mut lsystem2points = Lsystem2Points::new();

    for rule in rules {
        axiom2lsystem.add_rule(&rule);
        lsystem2points.add_rule(&rule);
    }

    let lsystem = axiom2lsystem.build(3).unwrap();
    println!("{lsystem}");

    let lsystem_points = lsystem2points.build_tree(lsystem);
    println!("{:?}", lsystem_points);
}
