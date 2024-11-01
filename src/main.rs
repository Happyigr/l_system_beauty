use std::{collections::VecDeque, ops::Sub};

use bevy::{ecs::component, prelude::*, reflect::List};
use bevy_rapier2d::prelude::*;
use camera::{move_camera, zoom_camera};
use lsystem::{Axiom2Lsystem, LsystemTree};
use noisy_bevy::simplex_noise_2d;

use crate::{
    camera::setup_camera,
    lsystem::{Lsystem2Points, LsystemAction, Rule},
};

mod camera;
mod lsystem;
mod physiks;

const WIND_POWER: f32 = 30.0;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.init_resource::<Game>();

    app.add_systems(Startup, setup_camera);
    app.add_systems(Startup, spawn_floor);
    app.add_systems(Startup, spawn_tree);

    app.add_systems(Update, draw_tree_with_wind);

    app.add_systems(Update, (zoom_camera, move_camera));

    app.run();
}

fn spawn_tree(mut commands: Commands) {
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

    let lsystem = axiom2lsystem.build(4).unwrap();
    // println!("{lsystem}");

    let lsystem_tree = lsystem2points.build_tree(lsystem).unwrap();
    // println!("{:?}", lsystem_tree);

    commands.spawn(lsystem_tree);
}

struct QueuedBranch {
    id: usize,
    start_point: Vec2,
}
impl QueuedBranch {
    pub fn new(id: usize, start_point: Vec2) -> Self {
        Self { id, start_point }
    }
}

#[derive(Resource)]
struct Game {
    wind_point: Vec2,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            wind_point: Default::default(),
        }
    }
}

fn draw_tree_with_wind(
    mut draw: Gizmos,
    mut tree_q: Query<&mut LsystemTree>,
    mut game: ResMut<Game>,
) {
    let mut tree = tree_q.get_single_mut().unwrap();

    let mut queued_branches: VecDeque<QueuedBranch> =
        vec![QueuedBranch::new(0, Vec2::new(0., 0.))].into();

    let wind_speed = simplex_noise_2d(game.wind_point) * WIND_POWER;
    game.wind_point += Vec2::new(0.01, 0.01);
    println!("{}", wind_speed);

    while !queued_branches.is_empty() {
        let queued_branch = queued_branches.pop_front().unwrap();
        let (branch_id, start_point) = (queued_branch.id, queued_branch.start_point);
        let mut dots = vec![start_point];

        let branch = tree.branches.get_mut(&branch_id).unwrap();
        for branch_dot in branch.points.iter() {
            let pos = branch_dot.point + (wind_speed / branch.weight);

            if let Some(branches) = &branch_dot.branches {
                branches.into_iter().for_each(|new_branch_id| {
                    queued_branches.push_back(QueuedBranch::new(new_branch_id.clone(), pos));
                });
            }

            dots.push(pos);
        }
        draw.linestrip_2d(dots, Color::linear_rgba(0.5, 0.1, 0.1, 0.5));
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
