use bevy::prelude::*;
use lsystem::Axiom2Lsystem;

use crate::lsystem::{Lsystem2Points, LsystemAction, Rule};

mod lsystem;

fn main() {
    let mut app = App::new();

    app.run();

    let rules = vec![
        Rule::new('A', "AB".to_string(), LsystemAction::TurnLeft),
        Rule::new('B', "A".to_string(), LsystemAction::DrawForward),
    ];

    let mut axiom2lsystem = Axiom2Lsystem::new("A".to_string());
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
