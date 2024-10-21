use std::{collections::VecDeque, ops::Sub};

use bevy::{prelude::*, reflect::List};
use bevy_rapier2d::prelude::*;
use camera::{move_camera, zoom_camera};
use lsystem::{Axiom2Lsystem, LsystemTree, Vec2Branched};

use crate::{
    camera::setup_camera,
    lsystem::{Lsystem2Points, LsystemAction, Rule},
};

mod camera;
mod lsystem;
mod physiks;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.add_systems(Startup, setup_camera);
    app.add_systems(Startup, spawn_floor);
    app.add_systems(Update, spawn_tree);

    app.add_systems(Update, (zoom_camera, move_camera));

    app.run();
}

fn spawn_tree(mut gizmos: Gizmos) {
    // squares
    // let mut axiom2lsystem = Axiom2Lsystem::new("F".to_string());
    // let rules = vec![
    //     Rule::new('F', "F+G".to_string(), LsystemAction::DrawForward),
    //     Rule::new('G', "F-G".to_string(), LsystemAction::DrawForward),
    //     Rule::new('+', "+".to_string(), LsystemAction::TurnRight),
    //     Rule::new('-', "-".to_string(), LsystemAction::TurnLeft),
    // ];
    // tree
    let mut axiom2lsystem = Axiom2Lsystem::new("X".to_string());
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

    let mut lsystem2points = Lsystem2Points::new();

    for rule in rules {
        axiom2lsystem.add_rule(&rule);
        lsystem2points.add_rule(&rule);
    }

    let lsystem = axiom2lsystem.build(5).unwrap();
    // println!("{lsystem}");

    let lsystem_tree = lsystem2points.build_tree(lsystem).unwrap();
    // println!("{:?}", lsystem_tree);

    let mut queued_branches: VecDeque<(usize, Vec2)> = vec![(0, Vec2::new(0., 0.))].into();

    draw_branch(&lsystem_tree, &mut gizmos, &mut queued_branches);
}

fn draw_branch(
    tree: &LsystemTree,
    draw: &mut Gizmos,
    queued_branches: &mut VecDeque<(usize, Vec2)>,
) {
    while !queued_branches.is_empty() {
        let (branch_id, previous_point) = queued_branches.pop_front().unwrap();
        let mut previous_point = previous_point;
        for branch_dot in tree.branches.get(&branch_id).unwrap() {
            if let Some(branches) = &branch_dot.branches {
                branches.into_iter().for_each(|new_branch_id| {
                    queued_branches.push(Box::new((new_branch_id.clone(), branch_dot.point)));
                });
            }

            draw.line_2d(
                previous_point,
                branch_dot.point,
                Color::linear_rgb(0.5, 0., 0.),
            );

            previous_point = branch_dot.point;
        }
        draw.circle_2d(previous_point, 10., Color::linear_rgb(0.1, 0.5, 0.0));
    }
}

fn spawn_floor(mut commands: Commands) {
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -25.0, 0.0)));
}
fn spawn_ball(mut commands: Commands) {
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Velocity {
            linvel: Vec2::new(1.0, 2.0),
            angvel: 0.2,
        })
        .insert(GravityScale(2.))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}
